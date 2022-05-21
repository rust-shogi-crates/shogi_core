#[cfg(feature = "alloc")]
use crate::Position;
use crate::{Bitboard, IllegalMoveKind, Move, PartialPosition, Piece, Square};

/// The status of a position.
///
/// Note that this type does not represent how a game finished:
/// for example, it cannot represent resignation and aborting of games.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

impl_ord_for_fieldless_enum!(PositionStatus);
impl_hash_for_fieldless_enum!(PositionStatus);

/// A trait that handles legality checking.
///
/// This crate does not provide any implementors of [`LegalityChecker`]:
/// users of this trait should depend on a crate that has an implementor of [`LegalityChecker`].
pub trait LegalityChecker {
    // status checking

    /// Returns the status of this position.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn status(&self, position: &Position) -> PositionStatus;
    /// Returns the status of this position.
    ///
    /// Because [`PartialPosition`] does not have a move sequence in it,
    /// it cannot return [`PositionStatus::Draw`] (which needs repetition check).
    fn status_partial(&self, position: &PartialPosition) -> PositionStatus;

    // legality checking

    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// If `is_legal` returns `Ok(())`, `position.make_move(mv)` must succeed.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn is_legal(&self, position: &Position, mv: Move) -> Result<(), IllegalMoveKind> {
        if self.status(position) != PositionStatus::InProgress {
            return Err(IllegalMoveKind::GameFinished);
        }
        self.is_legal_partial(position.inner(), mv)
    }
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// If `is_legal_lite` returns `true`, `position.make_move(mv)` must succeed.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn is_legal_lite(&self, position: &Position, mv: Move) -> bool {
        if self.status(position) != PositionStatus::InProgress {
            return false;
        }
        self.is_legal_partial_lite(position.inner(), mv)
    }
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// If `is_legal_partial` returns `Ok(())`, `position.make_move(mv)` must succeed.
    fn is_legal_partial(&self, position: &PartialPosition, mv: Move)
        -> Result<(), IllegalMoveKind>;
    /// Finds if a move is legal in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// If `is_legal_partial_lite` returns `true`, `position.make_move(mv)` must succeed.
    fn is_legal_partial_lite(&self, position: &PartialPosition, mv: Move) -> bool;

    // Enumeration of legal moves

    /// Finds all legal moves in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn all_legal_moves(&self, position: &Position) -> alloc::vec::Vec<Move> {
        if self.status(position) != PositionStatus::InProgress {
            return alloc::vec::Vec::new();
        }
        self.all_legal_moves_partial(position.inner())
    }
    /// Finds all legal moves in the given position.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn all_legal_moves_partial(&self, position: &PartialPosition) -> alloc::vec::Vec<Move>;
    /// Finds all legal normal moves in the given position that move a piece from `from` to some square.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// This function returns a [`Bitboard`] consisting of all destination squares.
    fn normal_from_candidates(&self, position: &PartialPosition, from: Square) -> Bitboard;
    /// Finds all legal normal moves in the given position that move `piece` from some square to `to`.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// This function returns a [`Bitboard`] consisting of all source squares.
    fn normal_to_candidates(
        &self,
        position: &PartialPosition,
        to: Square,
        piece: Piece,
    ) -> Bitboard;
    /// Finds all legal drop moves in the given position that drop `piece`.
    ///
    /// If `position` is not in progress, no moves are considered to be legal.
    /// This function returns a [`Bitboard`] consisting of all destination squares.
    fn drop_candidates(&self, position: &PartialPosition, piece: Piece) -> Bitboard;
    /// Makes a move. Returns `Ok(())` if successful.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn make_move(&self, position: &mut Position, mv: Move) -> Result<(), IllegalMoveKind> {
        self.is_legal(position, mv)?;
        // will always be Some(())
        let result = position.make_move(mv);
        debug_assert_eq!(result, Some(()));
        Ok(())
    }
}
