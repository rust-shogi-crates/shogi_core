use core::num::NonZeroU16;

use crate::{c_compat::OptionSquare, Piece, Square, ToUsi};

/// A move.
///
/// Because [`Move`] is cheap to copy, it implements [`Copy`].
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Move {
    /// A normal move, where a piece on a square is moved to another square.
    /// You can choose to promote a piece if certain conditions are met.
    Normal {
        /// The source square.
        from: Square,
        /// The destination square.
        to: Square,
        /// Whether this piece is promoted.
        promote: bool,
    },
    /// A drop move, where a piece is placed from a player's hand to a vacant square.
    Drop {
        /// The kind of piece to be placed.
        /// This field is necessary as otherwise there is no telling what kind of piece is placed.
        piece: Piece,
        /// The destination square.
        to: Square,
    },
}

impl Move {
    /// Finds the `from` square, if it exists.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Move, Piece, PieceKind, Square};
    /// assert_eq!(Move::Normal { from: Square::SQ_1B, to: Square::SQ_3D, promote: false}.from(), Some(Square::SQ_1B));
    /// assert_eq!(Move::Drop { piece: Piece::B_P, to: Square::SQ_3D }.from(), None);
    /// ```
    pub fn from(self) -> Option<Square> {
        match self {
            Move::Normal { from, .. } => Some(from),
            Move::Drop { .. } => None,
        }
    }

    /// Finds the `to` square.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Move, Piece, PieceKind, Square};
    /// assert_eq!(Move::Normal { from: Square::SQ_1B, to: Square::SQ_3D, promote: false}.to(), Square::SQ_3D);
    /// assert_eq!(Move::Drop { piece: Piece::B_P, to: Square::SQ_4E }.to(), Square::SQ_4E);
    /// ```
    pub fn to(self) -> Square {
        match self {
            Move::Normal { to, .. } => to,
            Move::Drop { to, .. } => to,
        }
    }

    /// Finds whether `self` promotes a piece.
    pub fn is_promoting(self) -> bool {
        matches!(self, Move::Normal { promote: true, .. })
    }

    /// Finds whether `self` is a drop move.
    #[inline]
    pub fn is_drop(self) -> bool {
        matches!(self, Move::Drop { .. })
    }
}

#[cfg(feature = "ord")]
#[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
impl core::cmp::PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "ord")]
#[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
impl core::cmp::Ord for Move {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (*self, *other) {
            (Move::Normal { .. }, Move::Drop { .. }) => core::cmp::Ordering::Less,
            (Move::Drop { .. }, Move::Normal { .. }) => core::cmp::Ordering::Greater,
            (
                Move::Normal {
                    from: from1,
                    to: to1,
                    promote: promote1,
                },
                Move::Normal {
                    from: from2,
                    to: to2,
                    promote: promote2,
                },
            ) => (from1, to1, promote1).cmp(&(from2, to2, promote2)),
            (
                Move::Drop {
                    piece: piece1,
                    to: to1,
                },
                Move::Drop {
                    piece: piece2,
                    to: to2,
                },
            ) => (piece1, to1).cmp(&(piece2, to2)),
        }
    }
}

#[cfg(feature = "hash")]
#[cfg_attr(docsrs, doc(cfg(feature = "hash")))]
impl core::hash::Hash for Move {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match *self {
            Move::Normal { from, to, promote } => {
                from.hash(state);
                to.hash(state);
                promote.hash(state);
            }
            Move::Drop { piece, to } => {
                piece.hash(state);
                to.hash(state);
            }
        }
    }
}

/// USI representation of a move.
///
/// Examples:
/// ```
/// # use shogi_core::{Move, Piece, Square, ToUsi};
/// let mv = Move::Normal { from: Square::SQ_7G, to: Square::SQ_7F, promote: false };
/// assert_eq!(mv.to_usi_owned(), "7g7f".to_owned());
/// let mv = Move::Normal { from: Square::SQ_8H, to: Square::SQ_2B, promote: true };
/// assert_eq!(mv.to_usi_owned(), "8h2b+".to_owned());
/// let mv = Move::Drop { piece: Piece::B_S, to: Square::SQ_5B };
/// assert_eq!(mv.to_usi_owned(), "S*5b".to_owned());
/// ```
/// Since: 0.1.4
impl ToUsi for Move {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        match *self {
            Move::Normal { from, to, promote } => {
                from.to_usi(sink)?;
                to.to_usi(sink)?;
                if promote {
                    // Safety: b'+' is an ASCII byte
                    unsafe { crate::common::write_ascii_byte(sink, b'+') }?;
                }
            }
            Move::Drop { piece, to } => {
                let piece_kind = piece.piece_kind();
                piece_kind.to_usi(sink)?;
                // Safety: b'*' is an ASCII byte
                unsafe { crate::common::write_ascii_byte(sink, b'*') }?;
                to.to_usi(sink)?;
            }
        }
        Ok(())
    }
}

/// A move packed in two bytes. C-compatible version of [`Move`].
///
/// Representation is as follows:
/// - normal move: promote * 32768 + from * 256 + to
/// - drop move: piece * 256 + 128 + to
///
/// Note that the representation cannot be zero.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CompactMove(NonZeroU16);

impl From<Move> for CompactMove {
    fn from(mv: Move) -> Self {
        let value = match mv {
            Move::Normal { from, to, promote } => {
                (promote as u16) << 15 | (from.index() as u16) << 8 | to.index() as u16
            }
            Move::Drop { piece, to } => (piece.as_u8() as u16) << 8 | 128 | to.index() as u16,
        };
        // Safety: value != 0 is implied from to.index() != 0
        Self(unsafe { NonZeroU16::new_unchecked(value) })
    }
}

impl From<CompactMove> for Move {
    fn from(mv: CompactMove) -> Self {
        let to = mv.to();
        let inner = mv.0.get();
        if mv.is_drop() {
            // a drop move
            let piece = (inner >> 8) as u8;
            let piece = unsafe { Piece::from_u8_unchecked(piece) };
            Move::Drop { piece, to }
        } else {
            let from = ((inner >> 8) & 127) as u8;
            // Safety: for all valid `CompactMove` which is normal, the part masked by 0x7f00 represents a valid square.
            let from = unsafe { Square::from_u8_unchecked(from) };
            let promote = (inner & 32768) != 0;
            Move::Normal { from, to, promote }
        }
    }
}

impl CompactMove {
    /// Creates a normal move.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{CompactMove, Move, Square};
    /// let from = Square::SQ_1B;
    /// let to = Square::SQ_3D;
    /// let promote = false;
    /// assert_eq!(<CompactMove as From<Move>>::from(Move::Normal { from, to, promote }), CompactMove::normal(from, to, promote));
    /// ```
    #[export_name = "CompactMove_normal"]
    pub extern "C" fn normal(from: Square, to: Square, promote: bool) -> Self {
        let value = (promote as u16) << 15 | (from.index() as u16) << 8 | to.index() as u16;
        // Safety: value != 0 is implied from to.index() != 0
        Self(unsafe { NonZeroU16::new_unchecked(value) })
    }

    /// Creates a drop move.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, CompactMove, Move, Piece, PieceKind, Square};
    /// let piece = Piece::W_G;
    /// let to = Square::SQ_3D;
    /// assert_eq!(<CompactMove as From<Move>>::from(Move::Drop { piece, to }), CompactMove::drop(piece, to));
    /// ```
    #[export_name = "CompactMove_drop"]
    pub extern "C" fn drop(piece: Piece, to: Square) -> Self {
        let value = (piece.as_u8() as u16) << 8 | 128 | to.index() as u16;
        // Safety: value != 0 is implied from to.index() != 0
        Self(unsafe { NonZeroU16::new_unchecked(value) })
    }

    /// Finds the `from` square, if it exists.
    pub fn from(self) -> Option<Square> {
        let inner = self.0.get();
        if self.is_drop() {
            // a drop move
            None
        } else {
            let from = ((inner >> 8) & 127) as u8;
            // Safety: for all valid `CompactMove` which is normal, the part masked by 0x7f00 represents a valid square.
            Some(unsafe { Square::from_u8_unchecked(from) })
        }
    }

    /// C interface of [`CompactMove::from`].
    #[no_mangle]
    pub extern "C" fn CompactMove_from(self) -> OptionSquare {
        self.from().into()
    }

    /// Finds the `to` square.
    #[export_name = "CompactMove_to"]
    pub extern "C" fn to(self) -> Square {
        let to = (self.0.get() & 127) as u8;
        // Safety: for all valid `CompactMove`, the least 7 bits represent a valid square.
        unsafe { Square::from_u8_unchecked(to) }
    }

    /// Finds whether `self` promotes a piece.
    #[export_name = "CompactMove_is_promoting"]
    pub extern "C" fn is_promoting(self) -> bool {
        (self.0.get() & 32768) != 0
    }

    /// Finds whether `self` is a drop move.
    #[export_name = "CompactMove_is_drop"]
    #[inline]
    pub extern "C" fn is_drop(self) -> bool {
        (self.0.get() & 128) != 0
    }
}

impl_ord_for_single_field!(CompactMove);
impl_hash_for_single_field!(CompactMove);

/// C-compatible type for <code>[Option]<[CompactMove]></code>.
///
/// cbindgen cannot deduce that <code>[Option]<[CompactMove]></code> can be represented by `uint16_t` in C, so we need to define the bridge type.
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct OptionCompactMove(u16);

impl From<Option<CompactMove>> for OptionCompactMove {
    #[inline(always)]
    fn from(arg: Option<CompactMove>) -> Self {
        Self(match arg {
            Some(result) => result.0.get(),
            None => 0,
        })
    }
}

impl From<OptionCompactMove> for Option<CompactMove> {
    #[inline(always)]
    fn from(arg: OptionCompactMove) -> Self {
        Some(CompactMove(NonZeroU16::new(arg.0)?))
    }
}

impl_ord_for_single_field!(OptionCompactMove);
impl_hash_for_single_field!(OptionCompactMove);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_works() {
        // normal moves
        for from in 1..=81 {
            let from = unsafe { Square::from_u8_unchecked(from) };
            for to in 1..=81 {
                let to = unsafe { Square::from_u8_unchecked(to) };
                for &promote in &[false, true] {
                    let mv = Move::Normal { from, to, promote };
                    let compact: CompactMove = mv.into();
                    let mv2: Move = compact.into();
                    assert_eq!(mv, mv2);
                }
            }
        }
        // drop moves
        for piece in Piece::all() {
            for to in 1..=81 {
                let to = unsafe { Square::from_u8_unchecked(to) };
                let mv = Move::Drop { piece, to };
                let compact: CompactMove = mv.into();
                let mv2: Move = compact.into();
                assert_eq!(mv, mv2);
            }
        }
    }

    #[test]
    fn normal_works() {
        for from in Square::all() {
            for to in Square::all() {
                for promote in [false, true] {
                    let cmv = CompactMove::normal(from, to, promote);
                    assert_eq!(
                        <CompactMove as From<Move>>::from(Move::Normal { from, to, promote }),
                        cmv,
                    );
                    assert_eq!(cmv.from(), Some(from));
                    assert_eq!(cmv.to(), to);
                    assert_eq!(cmv.is_promoting(), promote);
                    assert!(!cmv.is_drop());
                }
            }
        }
    }

    #[test]
    fn drop_works() {
        for piece in Piece::all() {
            for to in Square::all() {
                let cmv = CompactMove::drop(piece, to);
                assert_eq!(
                    <CompactMove as From<Move>>::from(Move::Drop { piece, to }),
                    cmv,
                );
                assert_eq!(cmv.from(), None);
                assert_eq!(cmv.to(), to);
                assert!(!cmv.is_promoting());
                assert!(cmv.is_drop());
            }
        }
    }
}
