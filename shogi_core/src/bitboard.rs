use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::{c_compat::OptionSquare, Square};

/// A subset of all squares.
///
/// Because [`Bitboard`] is cheap to copy, it implements [`Copy`].
/// Its [`Default`] value is an empty instance.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
// Valid representation: self.0[0] >> 63 and self.0[1] >> 18 must be equal to 0.
pub struct Bitboard([u64; 2]);

impl Bitboard {
    /// Creates an empty [`Bitboard`].
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Bitboard;
    /// let empty = Bitboard::empty();
    /// assert_eq!(empty.count(), 0);
    /// ```
    /// `const`: since 0.1.3
    pub const fn empty() -> Self {
        Self([0; 2])
    }

    /// C interface to [`Bitboard::empty`].
    #[no_mangle]
    pub extern "C" fn Bitboard_empty() -> Self {
        Self::empty()
    }

    /// Creates a [`Bitboard`] with a single element.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// assert_eq!(sq11.count(), 1);
    /// ```
    /// `const`: since 0.1.3
    pub const fn single(square: Square) -> Self {
        let index = square.array_index();
        let inner = if index < 63 {
            [1 << index, 0]
        } else {
            [0, 1 << (index - 63)]
        };
        Self(inner)
    }

    /// C interface to [`Bitboard::single`].
    #[no_mangle]
    pub extern "C" fn Bitboard_single(square: Square) -> Self {
        Self::single(square)
    }

    /// Finds how many elements this [`Bitboard`] has.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// let sq55 = Bitboard::single(Square::SQ_5E);
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
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// let sq55 = Bitboard::single(Square::SQ_5E);
    /// assert!(!(sq11 | sq55).is_empty());
    /// assert!(Bitboard::empty().is_empty());
    /// ```
    #[export_name = "Bitboard_is_empty"]
    pub extern "C" fn is_empty(self) -> bool {
        self.0 == [0; 2]
    }

    /// Finds if `self` as a subset contains a [`Square`].
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// assert!(sq11.contains(Square::SQ_1A));
    /// assert!(!sq11.contains(Square::SQ_9I));
    /// ```
    #[export_name = "Bitboard_contains"]
    pub extern "C" fn contains(self, square: Square) -> bool {
        let index = square.index() - 1;
        let overlap = if index < 63 {
            self.0[0] & 1 << index
        } else {
            self.0[1] & 1 << (index - 63)
        };
        overlap != 0
    }

    /// Finds the flipped version of `self`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// let sq99 = Bitboard::single(Square::SQ_9I);
    /// assert_eq!(sq11.flip(), sq99);
    /// ```
    #[export_name = "Bitboard_flip"]
    pub extern "C" fn flip(self) -> Self {
        let fst_rev = ((self.0[0] >> 17) | (self.0[1] << 46)) & !1;
        let snd_rev = self.0[0] << 46;
        let returned = [fst_rev.reverse_bits(), snd_rev.reverse_bits()];
        Self(returned)
    }

    /// If `self` is not empty, find a [`Square`] in `self` and returns it, removing it from `self`.
    ///
    /// The returned value is unspecified. It is guaranteed that the returned [`Square`] is a member of `self`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Bitboard, Square};
    /// let sq11 = Bitboard::single(Square::SQ_1A);
    /// let sq99 = Bitboard::single(Square::SQ_9I);
    /// let mut bitboard = sq11 | sq99;
    /// assert!(bitboard.pop().is_some());
    /// assert!(bitboard.pop().is_some());
    /// assert!(bitboard.pop().is_none()); // after `pop`ping twice `bitboard` becomes empty
    /// assert!(bitboard.is_empty());
    /// ```
    pub fn pop(&mut self) -> Option<Square> {
        if self.0[0] != 0 {
            let index = self.0[0].trailing_zeros() + 1;
            // Safety: 1 <= index <= 63
            let square = unsafe { Square::from_u8_unchecked(index as u8) };
            debug_assert!(self.contains(square));
            *self ^= square;
            return Some(square);
        }
        if self.0[1] == 0 {
            return None;
        }
        let index = self.0[1].trailing_zeros() + 63 + 1;
        // Safety: `64 <= index <= 81` holds because `self.0[1] & 0x1ffff` is not zero
        let square = unsafe { Square::from_u8_unchecked(index as u8) };
        debug_assert!(self.contains(square));
        *self ^= square;
        Some(square)
    }

    /// C interface of [`Bitboard::pop`].
    #[no_mangle]
    pub extern "C" fn Bitboard_pop(&mut self) -> OptionSquare {
        self.pop().into()
    }

    /// Returns the inner representation of `self`.
    ///
    /// Inner representation of [`Bitboard`] is unstable;
    /// however, `Bitboard::from_u128_unchecked(bb.to_u128()) == bb` always holds.
    ///
    /// Since: 0.1.3
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        // As little endian
        (self.0[1] as u128) << 64 | self.0[0] as u128
    }

    /// Creates a [`Bitboard`] with the given inner representation.
    ///
    /// Inner representation of [`Bitboard`] is unstable;
    /// however, `Bitboard::from_u128_unchecked(bb.to_u128()) == bb` always holds.
    ///
    /// # Safety
    /// `a` must be a valid representation of a [`Bitboard`].
    ///
    /// Since: 0.1.3
    pub const unsafe fn from_u128_unchecked(a: u128) -> Self {
        let v0 = a as u64;
        let v1 = (a >> 64) as u64;
        Self([v0, v1])
    }

    /// Creates a new bitboard with a single file populated.
    ///
    /// # Safety
    /// 1 <= file <= 9, 0 <= pattern < 512
    ///
    /// Since: 0.1.3
    pub const unsafe fn from_file_unchecked(file: u8, pattern: u16) -> Self {
        let mut data = [0; 2];
        if file <= 7 {
            data[0] = (pattern as u64) << ((file - 1) * 9);
        } else {
            data[1] = (pattern as u64) << ((file - 8) * 9);
        }
        Self(data)
    }

    /// Finds the pattern in a file.
    ///
    /// # Safety
    /// 1 <= file <= 9
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Bitboard, Square};
    /// let bitboard = Bitboard::single(Square::SQ_7G);
    /// assert_eq!(unsafe { bitboard.get_file_unchecked(7) }, 1 << 6);
    /// let bitboard = Bitboard::single(Square::SQ_8G) | Bitboard::single(Square::SQ_8H);
    /// assert_eq!(unsafe { bitboard.get_file_unchecked(8) }, 1 << 7 | 1 << 6);
    /// ```
    /// Since: 0.1.3
    pub const unsafe fn get_file_unchecked(self, file: u8) -> u16 {
        let pattern = if file <= 7 {
            self.0[0] >> ((file - 1) * 9)
        } else {
            self.0[1] >> ((file - 8) * 9)
        };
        pattern as u16 & 0x1ff
    }

    /// Bitwise or.
    ///
    /// Since: 0.1.3
    pub const fn or(self, other: Self) -> Self {
        Self([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }

    /// Bitwise and.
    ///
    /// Since: 0.1.3
    pub const fn and(self, other: Self) -> Self {
        Self([self.0[0] & other.0[0], self.0[1] & other.0[1]])
    }

    /// Bitwise xor.
    ///
    /// Since: 0.1.3
    pub const fn xor(self, other: Self) -> Self {
        Self([self.0[0] ^ other.0[0], self.0[1] ^ other.0[1]])
    }

    /// Bitwise andnot (`!self & others`).
    ///
    /// Since: 0.1.3
    pub const fn andnot(self, other: Self) -> Self {
        Self([!self.0[0] & other.0[0], !self.0[1] & other.0[1]])
    }

    /// Byte-wise reversing.
    ///
    /// Since: 0.1.3
    pub const fn swap_bytes(self) -> ByteSwappedBitboard {
        ByteSwappedBitboard([self.0[1].swap_bytes(), self.0[0].swap_bytes()])
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
    (trait => $trait:ident, assign_trait => $assign_trait:ident, funname => $funname:ident, assign_funname => $assign_funname:ident, op => $op:tt,) => {
        impl $trait for Bitboard {
            type Output = Self;

            fn $funname(self, rhs: Self) -> Self::Output {
                Self([self.0[0] $op rhs.0[0], self.0[1] $op rhs.0[1]])
            }
        }
        // Supports reference types in favor of https://doc.rust-lang.org/std/ops/index.html
        impl $trait<&'_ Bitboard> for Bitboard {
            type Output = Bitboard;

            fn $funname(self, rhs: &Self) -> Self::Output {
                self $op *rhs
            }
        }
        impl $trait<Bitboard> for &'_ Bitboard {
            type Output = Bitboard;

            fn $funname(self, rhs: Bitboard) -> Self::Output {
                *self $op rhs
            }
        }
        impl $trait<&'_ Bitboard> for &'_ Bitboard {
            type Output = Bitboard;

            fn $funname(self, rhs: &Bitboard) -> Self::Output {
                *self $op *rhs
            }
        }
        impl $assign_trait for Bitboard {
            fn $assign_funname(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
        impl $assign_trait<&'_ Bitboard> for Bitboard {
            fn $assign_funname(&mut self, rhs: &Self) {
                *self = *self $op *rhs;
            }
        }
        impl $assign_trait<Square> for Bitboard {
            fn $assign_funname(&mut self, rhs: Square) {
                *self = *self $op Bitboard::single(rhs);
            }
        }
        impl $assign_trait<&'_ Square> for Bitboard {
            fn $assign_funname(&mut self, rhs: &Square) {
                *self = *self $op Bitboard::single(*rhs);
            }
        }
    };
}

define_bit_trait!(
    trait => BitAnd, assign_trait => BitAndAssign,
    funname => bitand, assign_funname => bitand_assign,
    op => &,
);
define_bit_trait!(
    trait => BitOr, assign_trait => BitOrAssign,
    funname => bitor, assign_funname => bitor_assign,
    op => |,
);
define_bit_trait!(
    trait => BitXor, assign_trait => BitXorAssign,
    funname => bitxor, assign_funname => bitxor_assign,
    op => ^,
);

// `cbindgen` cannot find exported functions that are generated by macros.
// We need to define them manually for cbindgen to find and make bindings of them.
#[doc(hidden)]
impl Bitboard {
    #[no_mangle]
    pub extern "C" fn Bitboard_bitand(a: Bitboard, b: Bitboard) -> Bitboard {
        a & b
    }
    #[no_mangle]
    pub extern "C" fn Bitboard_bitand_assign(a: &mut Bitboard, b: Bitboard) {
        *a &= b;
    }

    #[no_mangle]
    pub extern "C" fn Bitboard_bitor(a: Bitboard, b: Bitboard) -> Bitboard {
        a | b
    }
    #[no_mangle]
    pub extern "C" fn Bitboard_bitor_assign(a: &mut Bitboard, b: Bitboard) {
        *a |= b;
    }

    #[no_mangle]
    pub extern "C" fn Bitboard_bitxor(a: Bitboard, b: Bitboard) -> Bitboard {
        a ^ b
    }
    #[no_mangle]
    pub extern "C" fn Bitboard_bitxor_assign(a: &mut Bitboard, b: Bitboard) {
        *a ^= b;
    }
}

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
        Self([!self.0[0] & ((1 << 63) - 1), !self.0[1] & ((1 << 18) - 1)])
    }
}

impl Not for &'_ Bitboard {
    type Output = Bitboard;

    /// Returns the complementary subset of `self`.
    ///
    /// You can create a subset consisting of the entire board with `!Bitboard::empty()`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Bitboard;
    /// assert_eq!((!&Bitboard::empty()).count(), 81);
    /// ```
    fn not(self) -> Self::Output {
        !*self
    }
}

/// C interface of `Bitboard::not`.
#[no_mangle]
pub extern "C" fn Bitboard_not(a: Bitboard) -> Bitboard {
    !a
}

impl_ord_for_single_field!(Bitboard);
impl_hash_for_single_field!(Bitboard);

/// A [`Bitboard`] with its all bytes reversed.
///
/// Since: 0.1.3
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct ByteSwappedBitboard([u64; 2]);

impl ByteSwappedBitboard {
    /// Returns the inner representation of `self`.
    ///
    /// Inner representation of [`ByteSwappedBitboard`] is unstable;
    /// however, `ByteSwappedBitboard::from_u128_unchecked(swapped_bb.to_u128()) == bb` always holds.
    #[inline(always)]
    pub const fn to_u128(self) -> u128 {
        // As little endian
        (self.0[1] as u128) << 64 | self.0[0] as u128
    }

    /// Creates a [`Bitboard`] with the given inner representation.
    ///
    /// Inner representation of [`ByteSwappedBitboard`] is unstable;
    /// however, `ByteSwappedBitboard::from_u128_unchecked(swapped_bb.to_u128()) == bb` always holds.
    ///
    /// # Safety
    /// `a` must be a valid representation of a [`ByteSwappedBitboard`].
    pub const unsafe fn from_u128_unchecked(a: u128) -> Self {
        let v0 = a as u64;
        let v1 = (a >> 64) as u64;
        Self([v0, v1])
    }

    /// Bitwise or.
    ///
    /// Since: 0.1.3
    pub const fn or(self, other: Self) -> Self {
        Self([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }

    /// Bitwise and.
    ///
    /// Since: 0.1.3
    pub const fn and(self, other: Self) -> Self {
        Self([self.0[0] & other.0[0], self.0[1] & other.0[1]])
    }

    /// Bitwise xor.
    ///
    /// Since: 0.1.3
    pub const fn xor(self, other: Self) -> Self {
        Self([self.0[0] ^ other.0[0], self.0[1] ^ other.0[1]])
    }

    /// Bitwise andnot (`!self & others`).
    ///
    /// Since: 0.1.3
    pub const fn andnot(self, other: Self) -> Self {
        Self([!self.0[0] & other.0[0], !self.0[1] & other.0[1]])
    }

    /// Byte-wise reversing.
    ///
    /// Since: 0.1.3
    #[inline(always)]
    pub const fn swap_bytes(self) -> Bitboard {
        Bitboard([self.0[1].swap_bytes(), self.0[0].swap_bytes()])
    }
}

impl_ord_for_single_field!(ByteSwappedBitboard);
impl_hash_for_single_field!(ByteSwappedBitboard);

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

    #[test]
    fn pop_works() {
        for square in Square::all() {
            let mut bitboard = Bitboard::single(square);
            assert_eq!(bitboard.pop(), Some(square));
            assert!(bitboard.is_empty());
        }
        for sq1 in Square::all() {
            for sq2 in Square::all() {
                if sq1 == sq2 {
                    continue;
                }
                let mut bitboard = Bitboard::single(sq1) | Bitboard::single(sq2);
                let result1 = bitboard.pop().unwrap();
                let result2 = bitboard.pop().unwrap();
                assert!((result1, result2) == (sq1, sq2) || (result1, result2) == (sq2, sq1));
                assert!(bitboard.is_empty());
            }
        }
    }

    #[test]
    fn from_file_works() {
        for file in 1..=9 {
            for pattern in 0..512 {
                let result = unsafe { Bitboard::from_file_unchecked(file, pattern) };
                let mut inner = 0;
                for rank in 1..=9 {
                    if result.contains(Square::new(file, rank).unwrap()) {
                        inner |= 1 << (rank - 1);
                    }
                }
                assert_eq!(pattern, inner);
                assert_eq!(result.count() as u32, pattern.count_ones());
            }
        }
    }
}
