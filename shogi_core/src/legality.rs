#[cfg(feature = "alloc")]
use crate::Position;
use crate::{Move, PartialPosition};

/// The status of a game.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub enum GameStatus {
    /// White's king was mated or white resigned.
    BlackWin = 1,
    /// Black's king was mated or black resigned.
    WhiteWin = 2,
    /// This can happen if e.g. `持将棋` (*jishōgi*) happens.
    Draw = 3,
    /// This can happen if e.g. `千日手` (*sennichite*, repetition) happens.
    Rematch = 4,
    /// A game is in progress.
    InProgress = 5,
    /// Invalid. A game contains illegal moves.
    Invalid = 6,
}

/// A trait that handles legality checking.
///
/// This crate does not provide any implementors of `LegalityChecker`:
/// users of this crate should depend on a crate that has an implementor of `LegalityChecker`.
pub trait LegalityChecker {
    #[cfg(feature = "alloc")]
    fn status(&self, position: &Position) -> GameStatus;
    fn status_partial(&self, position: &PartialPosition) -> GameStatus;
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    fn is_legal(&self, position: &Position, mv: Move) -> bool {
        if self.status(position) != GameStatus::InProgress {
            return false;
        }
        self.is_legal_partial(position.inner(), mv)
    }
    fn is_legal_partial(&self, position: &PartialPosition, mv: Move) -> bool;
    /// Finds all legal moves in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    fn all_legal_moves(&self, position: &Position) -> alloc::vec::Vec<Move> {
        if self.status(position) != GameStatus::InProgress {
            return alloc::vec::Vec::new();
        }
        self.all_legal_moves_partial(position.inner())
    }
    #[cfg(feature = "alloc")]
    fn all_legal_moves_partial(&self, position: &PartialPosition) -> alloc::vec::Vec<Move>;
    #[cfg(feature = "alloc")]
    fn make_move(&self, position: &mut Position, mv: Move) {
        if self.is_legal(position, mv) {
            // will always be Some(())
            let result = position.make_move(mv);
            debug_assert_eq!(result, Some(()));
        }
    }
}
