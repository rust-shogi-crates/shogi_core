use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::Square;

/// A subset of all squares.
///
/// Because `Bitboard` is cheap to copy, it implements [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
/// Its [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html) value is an empty instance.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
// Valid representation: self.0[1] >> 17 must be equal to 0.
pub struct Bitboard([u64; 2]);

impl Bitboard {
    /// Creates an empty `BitBoard`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Bitboard;
    /// let empty = Bitboard::empty();
    /// assert_eq!(empty.count(), 0);
    /// ```
    #[export_name = "Bitboard_empty"]
    pub extern "C" fn empty() -> Self {
        Self::default()
    }

    /// Creates a `BitBoard` with a single element.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
    /// assert_eq!(sq11.count(), 1);
    /// ```
    #[export_name = "Bitboard_single"]
    pub extern "C" fn single(square: Square) -> Self {
        let index = square.index() - 1;
        let value = 1 << (index % 64);
        let inner = if index < 64 { [value, 0] } else { [0, value] };
        Self(inner)
    }

    /// Finds how many elements this `Bitboard` has.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
    /// let sq55 = Bitboard::single(Square::new(5, 5).unwrap());
    /// assert_eq!((sq11 | sq55).count(), 2);
    /// ```
    #[export_name = "Bitboard_count"]
    pub extern "C" fn count(self) -> u8 {
        (self.0[0].count_ones() + self.0[1].count_ones()) as u8
    }

    /// Checks if `self` is an empty set.
    ///
    /// Equivalent to `self.count() == 0`, but this function is faster.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
    /// let sq55 = Bitboard::single(Square::new(5, 5).unwrap());
    /// assert!(!(sq11 | sq55).is_empty());
    /// assert!(Bitboard::empty().is_empty());
    /// ```
    #[export_name = "Bitboard_is_empty"]
    pub extern "C" fn is_empty(self) -> bool {
        self.0 == [0; 2]
    }

    /// Finds if `self` as a subset contains `square`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
    /// assert!(sq11.contains(Square::new(1, 1).unwrap()));
    /// assert!(!sq11.contains(Square::new(9, 9).unwrap()));
    /// ```
    #[export_name = "Bitboard_contains"]
    pub extern "C" fn contains(self, square: Square) -> bool {
        let index = square.index() - 1;
        let value = 1 << (index % 64);
        let overlap = if index < 64 {
            self.0[0] & value
        } else {
            self.0[1] & value
        };
        overlap != 0
    }

    /// Finds the flipped version of `self`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
    /// let sq99 = Bitboard::single(Square::new(9, 9).unwrap());
    /// assert_eq!(sq11.flip(), sq99);
    /// ```
    #[export_name = "Bitboard_flip"]
    pub extern "C" fn flip(self) -> Self {
        let fst_rev = (self.0[0] >> 17) | (self.0[1] << 47);
        let snd_rev = self.0[0] << 47;
        let returned = [fst_rev.reverse_bits(), snd_rev.reverse_bits()];
        Self(returned)
    }

    /// If `self` is not empty, find a square in `self` and returns it.
    ///
    /// The returned value is unspecified.
    pub fn pop(&mut self) -> Option<Square> {
        // TODO: optimize
        for i in 1..=81 {
            let square = unsafe { Square::from_u8_unchecked(i) };
            if self.contains(square) {
                *self ^= square;
                return Some(square);
            }
        }
        None
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(81))
    }
}

macro_rules! define_bit_trait {
    (trait => $trait:ident, assign_trait => $assign_trait:ident, funname => $funname:ident, assign_funname => $assign_funname:ident, op => $op:tt, exported_name => $exported_name:ident, assign_exported_name => $assign_exported_name:ident,) => {
        impl $trait for Bitboard {
            type Output = Self;

            fn $funname(self, rhs: Self) -> Self::Output {
                Self([self.0[0] $op rhs.0[0], self.0[1] $op rhs.0[1]])
            }
        }
        impl $assign_trait for Bitboard {
            fn $assign_funname(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
        impl $assign_trait<Square> for Bitboard {
            fn $assign_funname(&mut self, rhs: Square) {
                *self = *self $op Bitboard::single(rhs);
            }
        }

        #[no_mangle]
        pub extern "C" fn $exported_name(a: Bitboard, b: Bitboard) -> Bitboard {
            a $op b
        }
        #[no_mangle]
        pub extern "C" fn $assign_exported_name(a: &mut Bitboard, b: Bitboard) {
            a.$assign_funname(b);
        }
    };
}

define_bit_trait!(
    trait => BitAnd, assign_trait => BitAndAssign,
    funname => bitand, assign_funname => bitand_assign,
    op => &,
    exported_name => Bitboard_bitand, assign_exported_name => Bitboard_bitand_assign,
);
define_bit_trait!(
    trait => BitOr, assign_trait => BitOrAssign,
    funname => bitor, assign_funname => bitor_assign,
    op => |,
    exported_name => Bitboard_bitor, assign_exported_name => Bitboard_bitor_assign,
);
define_bit_trait!(
    trait => BitXor, assign_trait => BitXorAssign,
    funname => bitxor, assign_funname => bitxor_assign,
    op => ^,
    exported_name => Bitboard_bitxor, assign_exported_name => Bitboard_bitxor_assign,
);

impl Not for Bitboard {
    type Output = Self;

    /// Returns the complementary subset of `self`.
    ///
    /// You can create a subset consisting of the entire board with `!Bitboard::empty()`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Bitboard;
    /// assert_eq!((!Bitboard::empty()).count(), 81);
    /// ```
    fn not(self) -> Self::Output {
        Self([!self.0[0], !self.0[1] & ((1 << 17) - 1)])
    }
}

/// C interface of `Bitboard::not`.
#[no_mangle]
pub extern "C" fn Bitboard_not(a: Bitboard) -> Bitboard {
    !a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_works() {
        for file in 1..=9 {
            for rank in 1..=9 {
                let sq = Square::new(file, rank).unwrap();
                assert!(Bitboard::single(sq).contains(sq));
                for ofile in 1..=9 {
                    for orank in 1..=9 {
                        let osq = Square::new(ofile, orank).unwrap();
                        assert_eq!(Bitboard::single(sq).contains(osq), sq == osq);
                    }
                }
            }
        }
    }

    #[test]
    fn flip_works() {
        for file in 1..=9 {
            for rank in 1..=9 {
                let sq = Square::new(file, rank).unwrap();
                assert_eq!(Bitboard::single(sq).flip(), Bitboard::single(sq.flip()));
            }
        }
    }
}
