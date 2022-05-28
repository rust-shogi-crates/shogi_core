#![cfg_attr(not(test), no_std)] // Forbids using std::*.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod annotated_derive;

mod bitboard;
mod color;
mod common;
mod game_resolution;
mod hand;
mod illegal_move_kind;
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
pub use crate::game_resolution::GameResolution;

#[doc(inline)]
pub use crate::position::{PartialGame, PartialPosition};

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use crate::position::{Game, Position};

#[doc(inline)]
pub use crate::illegal_move_kind::IllegalMoveKind;

#[doc(inline)]
pub use crate::legality::{LegalityChecker, PositionStatus};

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

    #[doc(inline)]
    pub use crate::game_resolution::OptionGameResolution;

    #[doc(inline)]
    pub use crate::illegal_move_kind::ResultUnitIllegalMoveKind;
}

/// Constant values.
///
/// Since: 0.1.2
pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/piece_consts.rs"));
    include!(concat!(env!("OUT_DIR"), "/square_consts.rs"));
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
        assert_eq!(
            size_of::<Option<GameResolution>>(),
            size_of::<GameResolution>(),
        );
        assert_eq!(
            size_of::<Option<PartialPosition>>(),
            size_of::<PartialPosition>(),
        );
        assert_eq!(
            size_of::<Option<PositionStatus>>(),
            size_of::<PositionStatus>(),
        );
        assert_eq!(
            size_of::<Result<(), IllegalMoveKind>>(),
            size_of::<IllegalMoveKind>(),
        );
    }
}
