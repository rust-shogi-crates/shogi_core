use core::num::NonZeroU8;

use crate::{common, Color, ToUsi};

/// A square.
///
/// [`Square`] and <code>[Option]<[Square]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Square(NonZeroU8);

impl Square {
    /// Creates a new [`Square`] with given `file` and `rank`.
    ///
    /// `file` and `rank` must be between 1 and 9 (both inclusive).
    /// If this condition is not met, this function returns None.
    #[inline(always)]
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file.wrapping_sub(1) >= 9 || rank.wrapping_sub(1) >= 9 {
            return None;
        }
        // Safety: file >= 1 && rank >= 1 implies file * 9 + rank - 9 >= 1
        Some(Square(unsafe {
            NonZeroU8::new_unchecked(file * 9 + rank - 9)
        }))
    }

    /// C interface to [`Square::new`].
    #[no_mangle]
    pub extern "C" fn Square_new(file: u8, rank: u8) -> OptionSquare {
        Square::new(file, rank).into()
    }

    /// Creates a new [`Square`] with given `file`, `rank` and `color`.
    ///
    /// `file` and `rank` must be between 1 and 9 (both inclusive).
    /// If this condition is not met, this function returns None.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Color, Square};
    /// assert_eq!(Square::new_relative(3, 4, Color::Black), Some(Square::SQ_3D));
    /// assert_eq!(Square::new_relative(3, 4, Color::White), Some(Square::SQ_7F));
    /// ```
    pub const fn new_relative(file: u8, rank: u8, color: Color) -> Option<Self> {
        if file.wrapping_sub(1) >= 9 || rank.wrapping_sub(1) >= 9 {
            return None;
        }
        // Safety: file >= 1 && rank >= 1 implies 1 <= file * 9 + rank - 9 <= 81
        let relative_index = file * 9 + rank - 9;
        Some(Square(unsafe {
            NonZeroU8::new_unchecked(match color {
                Color::Black => relative_index,
                Color::White => 82 - relative_index,
            })
        }))
    }

    /// C interface to [`Square::new_relative`].
    #[no_mangle]
    pub extern "C" fn Square_new_relative(file: u8, rank: u8, color: Color) -> OptionSquare {
        Square::new_relative(file, rank, color).into()
    }

    /// Finds the file in range `1..=9`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::SQ_3D.file(), 3);
    /// ```
    #[inline(always)]
    #[export_name = "Square_file"]
    pub extern "C" fn file(self) -> u8 {
        self.sanity_check();
        (((self.0.get() + 8) as u32 * 57) >> 9) as u8
    }

    /// Finds the rank in range `1..=9`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::SQ_3D.rank(), 4);
    /// ```
    #[inline(always)]
    #[export_name = "Square_rank"]
    pub extern "C" fn rank(self) -> u8 {
        self.sanity_check();
        self.0.get() + 9 - 9 * self.file()
    }

    /// Finds the index of `self` in range `1..=81`.
    /// It is guaranteed that the result is equal to the internal representation, `9 * file + rank - 9`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::SQ_3D.index(), 22);
    /// ```
    #[inline(always)]
    #[export_name = "Square_index"]
    pub extern "C" fn index(self) -> u8 {
        self.sanity_check();
        self.0.get()
    }

    /// Finds the rank from the perspective of `color`.
    #[export_name = "Square_relative_rank"]
    pub extern "C" fn relative_rank(self, color: Color) -> u8 {
        let rank = self.rank();
        match color {
            Color::Black => rank,
            Color::White => 10 - rank,
        }
    }

    /// Finds the file from the perspective of `color`.
    #[export_name = "Square_relative_file"]
    pub extern "C" fn relative_file(self, color: Color) -> u8 {
        let file = self.file();
        match color {
            Color::Black => file,
            Color::White => 10 - file,
        }
    }

    /// Finds the reflected square of `self`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::SQ_1A.flip(), Square::SQ_9I);
    /// assert_eq!(Square::SQ_3D.flip(), Square::SQ_7F);
    /// ```
    #[inline(always)]
    #[export_name = "Square_flip"]
    pub extern "C" fn flip(self) -> Self {
        // Safety: self.0.get() is in range 1..=81.
        unsafe { Self::from_u8_unchecked(82 - self.0.get()) }
    }

    /// Converts a [`u8`] to a [`Square`]. If `value` is not in range `1..=81`, this function returns [`None`].
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::from_u8(21), Some(Square::SQ_3C));
    /// assert_eq!(Square::from_u8(0), None);
    /// assert_eq!(Square::from_u8(82), None);
    /// ```
    #[inline]
    #[export_name = "Square_from_u8"]
    pub extern "C" fn from_u8(value: u8) -> Option<Self> {
        // The shortest possible machine code for this function in x86_64 (System V AMD64 ABI) is:
        // 31 c0       xorl %eax, %eax
        // 40 80 ff 52 cmpb $82, %dil
        // 0f 42 c7    cmovbl %edi, %eax
        if matches!(value as u32, 0..=81) {
            // Safety: `value` is in `0..=81`, which is the range of valid representations.
            unsafe { core::mem::transmute::<_, OptionSquare>(value) }.into()
        } else {
            None
        }
    }

    /// Converts [`u8`] to [`Square`] without checking.
    ///
    /// # Safety
    /// `value` must be in range 1..=81
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        if !matches!(value, 1..=81) {
            core::hint::unreachable_unchecked();
        }
        Self(NonZeroU8::new_unchecked(value))
    }

    /// C interface to [`Square::from_u8_unchecked`].
    ///
    /// # Safety
    /// `value` must be in range 1..=81
    #[inline(always)]
    #[no_mangle]
    pub unsafe extern "C" fn Square_from_u8_unchecked(value: u8) -> Self {
        if !matches!(value, 1..=81) {
            core::hint::unreachable_unchecked();
        }
        Self(NonZeroU8::new_unchecked(value))
    }

    /// Shifts `self` by the given arguments. If the result would be out of the board, this function returns [`None`].
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Square;
    /// assert_eq!(Square::SQ_3C.shift(-1, 3), Some(Square::SQ_2F));
    /// assert_eq!(Square::SQ_8D.shift(0, -3), Some(Square::SQ_8A));
    /// assert_eq!(Square::SQ_3C.shift(-4, 3), None);
    /// ```
    #[export_name = "Square_shift"]
    pub extern "C" fn shift(self, file_delta: i8, rank_delta: i8) -> Option<Self> {
        self.sanity_check();
        let file_m1 = (self.file() as i8).wrapping_add(file_delta).wrapping_sub(1);
        let rank_m1 = (self.rank() as i8).wrapping_add(rank_delta).wrapping_sub(1);
        if !matches!(file_m1, 0..=8) || !matches!(rank_m1, 0..=8) {
            return None;
        }
        // Safety: 1 <= file_m1 + 1, rank_m1 + 1 <= 9
        Some(unsafe { Self::from_u8_unchecked((file_m1 * 9 + rank_m1 + 1) as u8) })
    }

    /// Returns the index of `self` for array accesses. This function returns an integer in range `0..Square::MAX`.
    ///
    /// Since: 0.1.2
    #[inline(always)]
    pub const fn array_index(self) -> usize {
        self.sanity_check();
        let result = (self.0.get() - 1) as usize;
        // Safety: result < Square::NUM always holds
        if result >= Self::NUM {
            unsafe { core::hint::unreachable_unchecked() };
        }
        result
    }

    /// How many elements should an array indexed by [`Square`] have?
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{PieceKind, Square};
    /// // values is long enough so values[square.index()] never panics
    /// let mut values = [None; Square::NUM];
    /// values[Square::SQ_5I.array_index()] = Some(PieceKind::King);
    /// ```
    /// Since: 0.1.2
    pub const NUM: usize = 81;

    /// Returns an iterator that iterates over all possible [`Square`]s
    /// in the ascending order of their indices.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::Square;
    /// assert_eq!(Square::all().count(), 81);
    /// ```
    pub fn all() -> impl core::iter::Iterator<Item = Self> {
        (1..=81).map(|index| unsafe { Self::from_u8_unchecked(index) })
    }

    // Check if self.0 is in 1..=81
    #[inline(always)]
    const fn sanity_check(self) {
        debug_assert!(matches!(self.0.get(), 1..=81));
        // Safety: for any valid Square, its representation must be in 1..=81.
        if !matches!(self.0.get(), 1..=81) {
            unsafe { core::hint::unreachable_unchecked() }
        }
    }
}

impl_ord_for_single_field!(Square);
impl_hash_for_single_field!(Square);

/// USI representation of a square.
///
/// Since: 0.1.4
impl ToUsi for Square {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        // Safety: '1'..='9' is always an ASCII byte
        unsafe { common::write_ascii_byte(sink, b'0' + self.file()) }?;
        // Safety: 'a'..='i' is always an ASCII byte
        unsafe { common::write_ascii_byte(sink, b'a' + self.rank() - 1) }?;
        Ok(())
    }
}

/// C interface of <code>[Option]<[Square]></code>.
///
/// This type is provided for C interoperability.
/// cbindgen cannot deduce that <code>[Option]<[Square]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
/// Users of this type should convert to/from <code>[Option]<[Square]></code>.
///
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OptionSquare(u8);

impl From<Option<Square>> for OptionSquare {
    #[inline(always)]
    fn from(arg: Option<Square>) -> Self {
        Self(match arg {
            Some(result) => result.0.get(),
            None => 0,
        })
    }
}

impl From<OptionSquare> for Option<Square> {
    #[inline(always)]
    fn from(arg: OptionSquare) -> Self {
        Some(Square(NonZeroU8::new(arg.0)?))
    }
}

impl_ord_for_single_field!(OptionSquare);
impl_hash_for_single_field!(OptionSquare);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        for file in 0..256 {
            for rank in 0..256 {
                let file = file as u8;
                let rank = rank as u8;
                let result = Square::new(file, rank);
                assert_eq!(
                    result.is_some(),
                    (1..=9).contains(&file) && (1..=9).contains(&rank),
                );
                if let Some(sq) = result {
                    assert_eq!(sq.file(), file);
                    assert_eq!(sq.rank(), rank);
                    assert_eq!(sq.relative_file(Color::Black), file);
                    assert_eq!(sq.relative_rank(Color::Black), rank);
                    assert_eq!(sq.relative_file(Color::White), 10 - file);
                    assert_eq!(sq.relative_rank(Color::White), 10 - rank);
                }
            }
        }
    }

    #[test]
    fn new_relative_works() {
        for file in 1..=9 {
            for rank in 1..=9 {
                let sq = Square::new_relative(file, rank, Color::Black).unwrap();
                assert_eq!(sq, Square::new(file, rank).unwrap());
                let sq = Square::new_relative(file, rank, Color::White).unwrap();
                assert_eq!(sq, Square::new(10 - file, 10 - rank).unwrap());
            }
        }
    }

    #[test]
    fn flip_works() {
        for file in 1..=9 {
            for rank in 1..=9 {
                let sq = Square::new(file, rank).unwrap();
                assert_eq!(sq.flip(), Square::new(10 - file, 10 - rank).unwrap());
            }
        }
    }

    // Reference implementation
    fn from_u8_reference(value: u8) -> Option<Square> {
        if matches!(value, 1..=81) {
            // Safety: `value` is in range `1..=81`.
            Some(unsafe { Square::from_u8_unchecked(value) })
        } else {
            None
        }
    }

    #[test]
    fn from_u8_works() {
        for value in 0..=255 {
            assert_eq!(Square::from_u8(value), from_u8_reference(value));
        }
    }

    // Reference implementation
    fn shift_reference(this: Square, file_delta: i8, rank_delta: i8) -> Option<Square> {
        // Computing in i32 to avoid overflow
        let file = this.file() as i32 + file_delta as i32;
        let rank = this.rank() as i32 + rank_delta as i32;
        if file <= 0 || rank <= 0 || file >= 10 || rank >= 10 {
            return None;
        }
        Square::new(file as u8, rank as u8)
    }

    #[test]
    fn shift_works() {
        for file in 1..=9 {
            for rank in 1..=9 {
                let sq = Square::new(file, rank).unwrap();
                // Exhaustive check: `shift` does not panic
                for file_delta in -128..127 {
                    for rank_delta in -128..127 {
                        let result = sq.shift(file_delta, rank_delta);
                        assert_eq!(result, shift_reference(sq, file_delta, rank_delta));
                    }
                }
            }
        }
    }
    #[test]
    fn array_index_works() {
        for (index, sq) in Square::all().enumerate() {
            assert_eq!(sq.array_index(), index);
        }
    }

    #[test]
    fn to_usi_works() {
        assert_eq!(Square::SQ_1A.to_usi_owned(), "1a".to_owned());
        assert_eq!(Square::SQ_1B.to_usi_owned(), "1b".to_owned());
        assert_eq!(Square::SQ_1C.to_usi_owned(), "1c".to_owned());
        assert_eq!(Square::SQ_1D.to_usi_owned(), "1d".to_owned());
        assert_eq!(Square::SQ_1E.to_usi_owned(), "1e".to_owned());
        assert_eq!(Square::SQ_1F.to_usi_owned(), "1f".to_owned());
        assert_eq!(Square::SQ_1G.to_usi_owned(), "1g".to_owned());
        assert_eq!(Square::SQ_1H.to_usi_owned(), "1h".to_owned());
        assert_eq!(Square::SQ_1I.to_usi_owned(), "1i".to_owned());
        assert_eq!(Square::SQ_2A.to_usi_owned(), "2a".to_owned());
        assert_eq!(Square::SQ_2B.to_usi_owned(), "2b".to_owned());
        assert_eq!(Square::SQ_2C.to_usi_owned(), "2c".to_owned());
        assert_eq!(Square::SQ_2D.to_usi_owned(), "2d".to_owned());
        assert_eq!(Square::SQ_2E.to_usi_owned(), "2e".to_owned());
        assert_eq!(Square::SQ_2F.to_usi_owned(), "2f".to_owned());
        assert_eq!(Square::SQ_2G.to_usi_owned(), "2g".to_owned());
        assert_eq!(Square::SQ_2H.to_usi_owned(), "2h".to_owned());
        assert_eq!(Square::SQ_2I.to_usi_owned(), "2i".to_owned());
        assert_eq!(Square::SQ_3A.to_usi_owned(), "3a".to_owned());
        assert_eq!(Square::SQ_3B.to_usi_owned(), "3b".to_owned());
        assert_eq!(Square::SQ_3C.to_usi_owned(), "3c".to_owned());
        assert_eq!(Square::SQ_3D.to_usi_owned(), "3d".to_owned());
        assert_eq!(Square::SQ_3E.to_usi_owned(), "3e".to_owned());
        assert_eq!(Square::SQ_3F.to_usi_owned(), "3f".to_owned());
        assert_eq!(Square::SQ_3G.to_usi_owned(), "3g".to_owned());
        assert_eq!(Square::SQ_3H.to_usi_owned(), "3h".to_owned());
        assert_eq!(Square::SQ_3I.to_usi_owned(), "3i".to_owned());
        assert_eq!(Square::SQ_4A.to_usi_owned(), "4a".to_owned());
        assert_eq!(Square::SQ_4B.to_usi_owned(), "4b".to_owned());
        assert_eq!(Square::SQ_4C.to_usi_owned(), "4c".to_owned());
        assert_eq!(Square::SQ_4D.to_usi_owned(), "4d".to_owned());
        assert_eq!(Square::SQ_4E.to_usi_owned(), "4e".to_owned());
        assert_eq!(Square::SQ_4F.to_usi_owned(), "4f".to_owned());
        assert_eq!(Square::SQ_4G.to_usi_owned(), "4g".to_owned());
        assert_eq!(Square::SQ_4H.to_usi_owned(), "4h".to_owned());
        assert_eq!(Square::SQ_4I.to_usi_owned(), "4i".to_owned());
        assert_eq!(Square::SQ_5A.to_usi_owned(), "5a".to_owned());
        assert_eq!(Square::SQ_5B.to_usi_owned(), "5b".to_owned());
        assert_eq!(Square::SQ_5C.to_usi_owned(), "5c".to_owned());
        assert_eq!(Square::SQ_5D.to_usi_owned(), "5d".to_owned());
        assert_eq!(Square::SQ_5E.to_usi_owned(), "5e".to_owned());
        assert_eq!(Square::SQ_5F.to_usi_owned(), "5f".to_owned());
        assert_eq!(Square::SQ_5G.to_usi_owned(), "5g".to_owned());
        assert_eq!(Square::SQ_5H.to_usi_owned(), "5h".to_owned());
        assert_eq!(Square::SQ_5I.to_usi_owned(), "5i".to_owned());
        assert_eq!(Square::SQ_6A.to_usi_owned(), "6a".to_owned());
        assert_eq!(Square::SQ_6B.to_usi_owned(), "6b".to_owned());
        assert_eq!(Square::SQ_6C.to_usi_owned(), "6c".to_owned());
        assert_eq!(Square::SQ_6D.to_usi_owned(), "6d".to_owned());
        assert_eq!(Square::SQ_6E.to_usi_owned(), "6e".to_owned());
        assert_eq!(Square::SQ_6F.to_usi_owned(), "6f".to_owned());
        assert_eq!(Square::SQ_6G.to_usi_owned(), "6g".to_owned());
        assert_eq!(Square::SQ_6H.to_usi_owned(), "6h".to_owned());
        assert_eq!(Square::SQ_6I.to_usi_owned(), "6i".to_owned());
        assert_eq!(Square::SQ_7A.to_usi_owned(), "7a".to_owned());
        assert_eq!(Square::SQ_7B.to_usi_owned(), "7b".to_owned());
        assert_eq!(Square::SQ_7C.to_usi_owned(), "7c".to_owned());
        assert_eq!(Square::SQ_7D.to_usi_owned(), "7d".to_owned());
        assert_eq!(Square::SQ_7E.to_usi_owned(), "7e".to_owned());
        assert_eq!(Square::SQ_7F.to_usi_owned(), "7f".to_owned());
        assert_eq!(Square::SQ_7G.to_usi_owned(), "7g".to_owned());
        assert_eq!(Square::SQ_7H.to_usi_owned(), "7h".to_owned());
        assert_eq!(Square::SQ_7I.to_usi_owned(), "7i".to_owned());
        assert_eq!(Square::SQ_8A.to_usi_owned(), "8a".to_owned());
        assert_eq!(Square::SQ_8B.to_usi_owned(), "8b".to_owned());
        assert_eq!(Square::SQ_8C.to_usi_owned(), "8c".to_owned());
        assert_eq!(Square::SQ_8D.to_usi_owned(), "8d".to_owned());
        assert_eq!(Square::SQ_8E.to_usi_owned(), "8e".to_owned());
        assert_eq!(Square::SQ_8F.to_usi_owned(), "8f".to_owned());
        assert_eq!(Square::SQ_8G.to_usi_owned(), "8g".to_owned());
        assert_eq!(Square::SQ_8H.to_usi_owned(), "8h".to_owned());
        assert_eq!(Square::SQ_8I.to_usi_owned(), "8i".to_owned());
        assert_eq!(Square::SQ_9A.to_usi_owned(), "9a".to_owned());
        assert_eq!(Square::SQ_9B.to_usi_owned(), "9b".to_owned());
        assert_eq!(Square::SQ_9C.to_usi_owned(), "9c".to_owned());
        assert_eq!(Square::SQ_9D.to_usi_owned(), "9d".to_owned());
        assert_eq!(Square::SQ_9E.to_usi_owned(), "9e".to_owned());
        assert_eq!(Square::SQ_9F.to_usi_owned(), "9f".to_owned());
        assert_eq!(Square::SQ_9G.to_usi_owned(), "9g".to_owned());
        assert_eq!(Square::SQ_9H.to_usi_owned(), "9h".to_owned());
        assert_eq!(Square::SQ_9I.to_usi_owned(), "9i".to_owned());
    }
}
