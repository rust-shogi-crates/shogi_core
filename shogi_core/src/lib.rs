#![cfg_attr(not(test), no_std)] // Forbids using std::*.
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

mod bitboard;
mod color;
mod common;
mod hand;
mod legality;
mod mv;
mod piece;
mod piece_kind;
mod position;
mod square;
mod to_usi;

#[doc(inline)]
pub use crate::to_usi::ToUsi;

#[doc(inline)]
pub use crate::color::Color;

#[doc(inline)]
pub use crate::square::Square;

#[doc(inline)]
pub use crate::piece_kind::PieceKind;

#[doc(inline)]
pub use crate::piece::Piece;

#[doc(inline)]
pub use crate::mv::Move;

#[doc(inline)]
pub use crate::mv::CompactMove;

#[doc(inline)]
pub use crate::hand::Hand;

#[doc(inline)]
pub use crate::bitboard::Bitboard;

#[doc(inline)]
pub use crate::position::PartialPosition;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use crate::position::Position;

#[doc(inline)]
pub use crate::legality::{GameStatus, LegalityChecker};

/// Types that are exposed to C.
pub mod c_compat {
    #[doc(inline)]
    pub use crate::piece_kind::OptionPieceKind;

    #[doc(inline)]
    pub use crate::piece::OptionPiece;

    #[doc(inline)]
    pub use crate::square::OptionSquare;

    #[doc(inline)]
    pub use crate::mv::OptionCompactMove;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminant_elision_works() {
        use core::mem::size_of;

        assert_eq!(size_of::<Option<Color>>(), size_of::<Color>());
        assert_eq!(size_of::<Option<Square>>(), size_of::<Square>());
        assert_eq!(size_of::<Option<PieceKind>>(), size_of::<PieceKind>());
        assert_eq!(size_of::<Option<Piece>>(), size_of::<Piece>());
    }
}
