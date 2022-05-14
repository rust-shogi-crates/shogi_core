#[cfg(feature = "alloc")]
use crate::Position;
use crate::{Bitboard, IllegalMoveKind, Move, PartialPosition, Piece, Square};

/// The status of a position.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub enum PositionStatus {
    /// White's king was mated.
    BlackWins = 1,
    /// Black's king was mated.
    WhiteWins = 2,
    /// Draw by repetition happened.
    Draw = 3,
    /// A game is in progress.
    InProgress = 4,
    /// Invalid. A game contains illegal moves or is in an inconsistent state.
    Invalid = 5,
}

/// A trait that handles legality checking.
///
/// This crate does not provide any implementors of `LegalityChecker`:
/// users of this crate should depend on a crate that has an implementor of `LegalityChecker`.
pub trait LegalityChecker {
    #[cfg(feature = "alloc")]
    fn status(&self, position: &Position) -> PositionStatus;
    fn status_partial(&self, position: &PartialPosition) -> PositionStatus;
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    fn is_legal(&self, position: &Position, mv: Move) -> Result<(), IllegalMoveKind> {
        if self.status(position) != PositionStatus::InProgress {
            return Err(IllegalMoveKind::GameFinished);
        }
        self.is_legal_partial(position.inner(), mv)
    }
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    fn is_legal_lite(&self, position: &Position, mv: Move) -> bool {
        if self.status(position) != PositionStatus::InProgress {
            return false;
        }
        self.is_legal_partial_lite(position.inner(), mv)
    }
    fn is_legal_partial(&self, position: &PartialPosition, mv: Move)
        -> Result<(), IllegalMoveKind>;
    fn is_legal_partial_lite(&self, position: &PartialPosition, mv: Move) -> bool;
    /// Finds all legal moves in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    fn all_legal_moves(&self, position: &Position) -> alloc::vec::Vec<Move> {
        if self.status(position) != PositionStatus::InProgress {
            return alloc::vec::Vec::new();
        }
        self.all_legal_moves_partial(position.inner())
    }
    #[cfg(feature = "alloc")]
    fn all_legal_moves_partial(&self, position: &PartialPosition) -> alloc::vec::Vec<Move>;
    fn normal_from_candidates(&self, position: &PartialPosition, from: Square) -> Bitboard;
    fn normal_to_candidates(
        &self,
        position: &PartialPosition,
        to: Square,
        piece: Piece,
    ) -> Bitboard;
    fn drop_candidates(&self, position: &PartialPosition, piece: Piece) -> Bitboard;
    #[cfg(feature = "alloc")]
    fn make_move(&self, position: &mut Position, mv: Move) -> Result<(), IllegalMoveKind> {
        self.is_legal(position, mv)?;
        // will always be Some(())
        let result = position.make_move(mv);
        debug_assert_eq!(result, Some(()));
        Ok(())
    }
}
