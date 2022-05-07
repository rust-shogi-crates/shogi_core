use core::num::NonZeroU16;

use crate::{Piece, Square};

/// A move.
///
/// Because `Move` is cheap to copy, it implements [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
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
    pub fn from(self) -> Option<Square> {
        match self {
            Move::Normal { from, .. } => Some(from),
            Move::Drop { .. } => None,
        }
    }

    /// Finds the `to` square.
    pub fn to(self) -> Square {
        match self {
            Move::Normal { to, .. } => to,
            Move::Drop { to, .. } => to,
        }
    }

    /// Finds whether `self` is a drop move.
    #[inline]
    pub fn is_drop(self) -> bool {
        matches!(self, Move::Drop { .. })
    }
}

/// A move packed in two bytes.
///
/// Representation is as follows:
/// - normal move: promote * 32768 + from * 256 + to
/// - drop move: piece * 256 + 128 + to
///
/// Note that the representation cannot be zero.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
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
            let piece = unsafe { Piece::from_u8(piece) };
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
    /// Finds the `from` square, if it exists.
    #[export_name = "CompactMove_from"]
    pub extern "C" fn from(self) -> Option<Square> {
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

/// <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
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
    fn from(arg: OptionCompactMove) -> Self {
        Some(CompactMove(NonZeroU16::new(arg.0)?))
    }
}

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
}
