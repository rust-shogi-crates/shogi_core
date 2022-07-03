use core::num::NonZeroU8;

use crate::common::write_ascii_byte;
use crate::{Color, PieceKind, ToUsi};

/// A piece + who owns it.
///
/// [`Piece`] and <code>[Option]<[Piece]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
///
/// Valid representations are `1..=14`, and `17..=30`. `1..=14` represents a black [`Piece`] and `17..=30` represents a white [`Piece`].
/// Examples:
/// ```
/// use shogi_core::{Color, Piece, PieceKind};
/// assert_eq!(core::mem::size_of::<Piece>(), 1);
/// assert!(Piece::B_P.as_u8() <= 14);
/// ```
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
// Internal representation: 1..=14: black, 17..=30: white
pub struct Piece(NonZeroU8);

/// C-compatible type for <code>[Option]<[Piece]></code> with defined representations.
///
/// Valid representations are `0..=14`, and `17..=30`. `0` represents [`None`], `1..=14` represents a black [`Piece`] and `17..=30` represents a white [`Piece`].
///
/// cbindgen cannot deduce that <code>[Option]<[Piece]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
/// See: <https://github.com/eqrion/cbindgen/issues/326>
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OptionPiece(u8);

impl Piece {
    /// Creates a new [`Piece`] from a [`PieceKind`] and a [`Color`].
    #[must_use]
    pub const fn new(piece_kind: PieceKind, color: Color) -> Self {
        let disc = piece_kind as u8;
        let value = disc
            + match color {
                Color::Black => 0,
                Color::White => 16,
            };
        // Safety: disc > 0 always holds
        Piece(unsafe { NonZeroU8::new_unchecked(value) })
    }
    /// C interface to [`Piece::new`].
    #[no_mangle]
    pub extern "C" fn Piece_new(piece_kind: PieceKind, color: Color) -> Self {
        Self::new(piece_kind, color)
    }
    /// An inverse of [`Piece::new`]. Finds a [`PieceKind`] and a [`Color`] from a [`Piece`].
    #[must_use]
    #[inline(always)]
    pub fn to_parts(self) -> (PieceKind, Color) {
        let data = self.0.get();
        let disc = data & 15;
        (
            // Safety: 1 <= disc <= 14
            unsafe { PieceKind::from_u8_unchecked(disc) },
            if data >= 16 {
                Color::White
            } else {
                Color::Black
            },
        )
    }
    /// Finds the [`PieceKind`] of this piece.
    #[must_use]
    #[export_name = "Piece_piece_kind"]
    #[inline(always)]
    pub extern "C" fn piece_kind(self) -> PieceKind {
        self.to_parts().0
    }
    /// Finds the [`Color`] of this piece.
    #[must_use]
    #[export_name = "Piece_color"]
    #[inline(always)]
    pub extern "C" fn color(self) -> Color {
        self.to_parts().1
    }

    /// Returns the internal representation.
    #[must_use]
    #[inline(always)]
    pub fn as_u8(self) -> u8 {
        self.0.get()
    }

    /// Promote a [`Piece`]. Same as [`PieceKind::promote`] with color.
    #[must_use]
    pub fn promote(self) -> Option<Piece> {
        let (piece_kind, color) = self.to_parts();
        Some(Self::new(piece_kind.promote()?, color))
    }

    /// C interface of [`Piece::promote`].
    #[no_mangle]
    #[inline(always)]
    pub extern "C" fn Piece_promote(self) -> OptionPiece {
        OptionPiece::from(self.promote())
    }

    /// Un-promote a [`Piece`]. Same as [`PieceKind::unpromote`] with color.
    #[must_use]
    pub fn unpromote(self) -> Option<Piece> {
        let (piece_kind, color) = self.to_parts();
        Some(Self::new(piece_kind.unpromote()?, color))
    }

    /// C interface of [`Piece::unpromote`].
    #[no_mangle]
    #[inline(always)]
    pub extern "C" fn Piece_unpromote(self) -> OptionPiece {
        OptionPiece::from(self.unpromote())
    }

    /// `value` must be in range 1..=14 or 17..=30.
    #[inline(always)]
    pub(crate) unsafe fn from_u8_unchecked(value: u8) -> Self {
        Self(NonZeroU8::new_unchecked(value))
    }

    /// Returns the index of `self` for array accesses. This function returns an integer in range `0..Piece::MAX`.
    ///
    /// This item is experimental: it is subject to change or deletion.
    #[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
    #[cfg(feature = "experimental")]
    #[inline]
    pub const fn array_index(self) -> usize {
        self.0.get() as usize - 1
    }

    /// How many elements should an array indexed by [`Piece`] have?
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Piece, PieceKind};
    /// // values is long enough so values[piece_kind.index()] never panics
    /// let mut values = [0; Piece::NUM];
    /// values[Piece::W_P.array_index()] = -10;
    /// values[Piece::B_L.array_index()] = 25;
    /// values[Piece::W_PR.array_index()] = -155;
    /// ```
    /// This item is experimental: it is subject to change or deletion.
    #[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
    #[cfg(feature = "experimental")]
    // Apery-style: non-contiguous, memory-consuming but fast
    // https://github.com/HiraokaTakuya/apery_rust/blob/v2.0.0/src/piecevalue.rs#L18-L50
    pub const NUM: usize = 31;

    /// Returns all possible [`Piece`]s.
    pub fn all() -> [Self; 28] {
        let mut result = [Self::new(PieceKind::Pawn, Color::Black); 28];
        let piece_kinds = PieceKind::all();
        let mut index = 0;
        let colors = Color::all();
        for &piece_kind in &piece_kinds {
            for &color in &colors {
                // Safety: 0 <= index < 28
                *unsafe { result.get_unchecked_mut(index) } = Piece::new(piece_kind, color);
                index += 1;
            }
        }
        result
    }
}

impl_ord_for_single_field!(Piece);
impl_hash_for_single_field!(Piece);

impl From<Option<Piece>> for OptionPiece {
    #[inline(always)]
    fn from(arg: Option<Piece>) -> Self {
        Self(match arg {
            Some(result) => result.0.get(),
            None => 0,
        })
    }
}

impl From<OptionPiece> for Option<Piece> {
    #[inline(always)]
    fn from(arg: OptionPiece) -> Self {
        Some(Piece(NonZeroU8::new(arg.0)?))
    }
}

impl_ord_for_single_field!(OptionPiece);
impl_hash_for_single_field!(OptionPiece);

impl ToUsi for Piece {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        let (piece_kind, color) = self.to_parts();

        match (piece_kind, color) {
            (piece_kind, color) if piece_kind as u8 >= PieceKind::ProPawn as u8 => {
                let table = b"+P+L+N+S+B+R+p+l+n+s+b+r";
                let index = match color {
                    Color::Black => 0,
                    Color::White => 6,
                };
                let index = index + piece_kind as usize - PieceKind::ProPawn as usize;
                debug_assert!(index < 12);
                // Safety: table has only ASCII bytes, index < 12
                sink.write_str(unsafe {
                    core::str::from_utf8_unchecked(table.get_unchecked(2 * index..2 * index + 2))
                })
            }
            (piece_kind, color) => {
                debug_assert!(piece_kind as u8 <= PieceKind::King as u8);
                let symbols = b"PLNSGBRKplnsgbrk";
                let offset = match color {
                    Color::Black => 0,
                    Color::White => 8,
                };
                // Safety: 1 <= offset + piece_kind <= 16
                let c = *unsafe { symbols.get_unchecked(offset + piece_kind as usize - 1) };
                // Safety: the written byte is in ASCII for every branch
                unsafe { write_ascii_byte(sink, c) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_parts_works() {
        let piece_kinds = PieceKind::all();
        let colors = Color::all();
        for &piece_kind in &piece_kinds {
            for &color in &colors {
                let piece = Piece::new(piece_kind, color);
                let (piece_kind0, color0) = piece.to_parts();
                assert_eq!(piece_kind0, piece_kind);
                assert_eq!(color0, color);
            }
        }
    }

    // reference implementation
    fn to_usi_reference<W: core::fmt::Write>(this: &Piece, sink: &mut W) -> core::fmt::Result {
        let (piece_kind, color) = this.to_parts();
        match (piece_kind, color) {
            (PieceKind::Pawn, Color::Black) => sink.write_char('P'),
            (PieceKind::Pawn, Color::White) => sink.write_char('p'),
            (PieceKind::Lance, Color::Black) => sink.write_char('L'),
            (PieceKind::Lance, Color::White) => sink.write_char('l'),
            (PieceKind::Knight, Color::Black) => sink.write_char('N'),
            (PieceKind::Knight, Color::White) => sink.write_char('n'),
            (PieceKind::Silver, Color::Black) => sink.write_char('S'),
            (PieceKind::Silver, Color::White) => sink.write_char('s'),
            (PieceKind::Gold, Color::Black) => sink.write_char('G'),
            (PieceKind::Gold, Color::White) => sink.write_char('g'),
            (PieceKind::Bishop, Color::Black) => sink.write_char('B'),
            (PieceKind::Bishop, Color::White) => sink.write_char('b'),
            (PieceKind::Rook, Color::Black) => sink.write_char('R'),
            (PieceKind::Rook, Color::White) => sink.write_char('r'),
            (PieceKind::King, Color::Black) => sink.write_char('K'),
            (PieceKind::King, Color::White) => sink.write_char('k'),
            (PieceKind::ProPawn, Color::Black) => sink.write_str("+P"),
            (PieceKind::ProPawn, Color::White) => sink.write_str("+p"),
            (PieceKind::ProLance, Color::Black) => sink.write_str("+L"),
            (PieceKind::ProLance, Color::White) => sink.write_str("+l"),
            (PieceKind::ProKnight, Color::Black) => sink.write_str("+N"),
            (PieceKind::ProKnight, Color::White) => sink.write_str("+n"),
            (PieceKind::ProSilver, Color::Black) => sink.write_str("+S"),
            (PieceKind::ProSilver, Color::White) => sink.write_str("+s"),
            (PieceKind::ProBishop, Color::Black) => sink.write_str("+B"),
            (PieceKind::ProBishop, Color::White) => sink.write_str("+b"),
            (PieceKind::ProRook, Color::Black) => sink.write_str("+R"),
            (PieceKind::ProRook, Color::White) => sink.write_str("+r"),
        }
    }

    #[test]
    fn to_usi_works() {
        for piece in Piece::all() {
            let mut actual = String::new();
            piece.to_usi(&mut actual).unwrap();
            let mut expected = String::new();
            to_usi_reference(&piece, &mut expected).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
