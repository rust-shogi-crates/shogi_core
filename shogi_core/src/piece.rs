use core::num::NonZeroU8;

use crate::{Color, PieceKind, ToUsi};

/// A piece + who owns it.
///
/// `Piece` and `Option<Piece>` are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
///
/// Examples:
/// ```
/// use shogi_core::Piece;
/// assert_eq!(core::mem::size_of::<Piece>(), 1);
/// ```
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
// Internal representation: 1..=14: black, 17..=30: white
pub struct Piece(NonZeroU8);

/// <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct OptionPiece(u8);

impl Piece {
    /// Creates a new `Piece` from `PieceKind` and `Color`.
    #[must_use]
    #[export_name = "Piece_new"]
    pub extern "C" fn new(piece_kind: PieceKind, color: Color) -> Self {
        let disc = piece_kind as u8;
        let value = disc
            + match color {
                Color::Black => 0,
                Color::White => 16,
            };
        // Safety: disc > 0 always holds
        Piece(unsafe { NonZeroU8::new_unchecked(value) })
    }
    /// An inverse of `new`. Finds `PieceKind` and `Color` from a `Piece`.
    #[must_use]
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
    /// Finds the `PieceKind` of this piece.
    #[must_use]
    #[export_name = "Piece_piece_kind"]
    pub extern "C" fn piece_kind(self) -> PieceKind {
        self.to_parts().0
    }
    /// Finds the `Color` of this piece.
    #[must_use]
    #[export_name = "Piece_color"]
    pub extern "C" fn color(self) -> Color {
        self.to_parts().1
    }

    /// Returns the internal representation.
    #[must_use]
    pub fn as_u8(self) -> u8 {
        self.0.get()
    }

    /// Promote a `Piece`. Same as `PieceKind::promote` with color.
    #[must_use]
    #[export_name = "Piece_promote"]
    pub extern "C" fn promote(self) -> Option<Piece> {
        let (piece_kind, color) = self.to_parts();
        Some(Self::new(piece_kind.promote()?, color))
    }

    /// Un-promote a `Piece`. Same as `PieceKind::unpromote` with color.
    #[must_use]
    #[export_name = "Piece_unpromote"]
    pub extern "C" fn unpromote(self) -> Option<Piece> {
        let (piece_kind, color) = self.to_parts();
        Some(Self::new(piece_kind.unpromote()?, color))
    }

    /// `value` must be in range 1..=14 or 17..=30.
    pub(crate) unsafe fn from_u8(value: u8) -> Self {
        Self(NonZeroU8::new_unchecked(value))
    }

    /// Returns all possible `Piece`s.
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
    fn from(arg: OptionPiece) -> Self {
        Some(Piece(NonZeroU8::new(arg.0)?))
    }
}

impl ToUsi for Piece {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        let (piece_kind, color) = self.to_parts();
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
}