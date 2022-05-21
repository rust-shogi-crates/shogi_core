use crate::common::{write_ascii_byte, write_u8};
use crate::{PieceKind, ToUsi};

/// A hand of a single player. A hand is a multiset of unpromoted pieces (except a king).
///
/// This type can hold up to 255 pieces of each kind, although the rule of shogi prohibits it.
///
/// Because [`Hand`] is cheap to copy, it implements [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
/// Its [`Default`] value is an empty instance.
#[repr(C)]
#[derive(Eq, Clone, Copy, Debug, Default)]
pub struct Hand([u8; 8]);

impl Hand {
    /// Creates an empty instance of [`Hand`].
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Hand;
    /// assert_eq!(Hand::new(), Hand::default());
    /// ```
    #[export_name = "Hand_new"]
    #[inline(always)]
    pub extern "C" fn new() -> Self {
        Self::default()
    }

    /// Find a new [`Hand`] with `piece_kind` added, if possible.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Hand, PieceKind};
    /// let hand = Hand::new().added(PieceKind::Pawn).unwrap();
    /// assert_eq!(hand.count(PieceKind::Pawn), Some(1));
    /// assert_eq!(hand.count(PieceKind::Rook), Some(0));
    /// ```
    ///
    /// `added` will always wraps around if the number exceeds 255:
    /// ```
    /// # use shogi_core::{Hand, PieceKind};
    /// let mut hand = Hand::new();
    /// for _ in 0..255 {
    ///     hand = hand.added(PieceKind::Pawn).unwrap();
    /// }
    ///
    /// // hand has 255 pawns.
    /// assert_eq!(hand.count(PieceKind::Pawn), Some(255));
    ///
    /// // We add one more pawn.
    /// let hand = hand.added(PieceKind::Pawn).unwrap();
    /// // Somehow, we lost all pawns!
    /// assert_eq!(hand.count(PieceKind::Pawn), Some(0));
    /// ```
    #[inline]
    pub fn added(mut self, piece_kind: PieceKind) -> Option<Hand> {
        let index = (piece_kind as u8 - 1) as usize;
        if index < 7 {
            self.0[index] = self.0[index].wrapping_add(1);
            return Some(self);
        }
        None
    }

    /// Find a new [`Hand`], with a single piece of `piece_kind` removed, if possible.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::{Hand, PieceKind};
    /// let hand = Hand::new().added(PieceKind::Pawn).unwrap();
    /// let hand = hand.added(PieceKind::Pawn).unwrap();
    /// let hand = hand.added(PieceKind::Pawn).unwrap();
    ///
    /// // Now we have three pawns.
    /// assert_eq!(hand.count(PieceKind::Pawn), Some(3));
    ///
    /// // We discard one pawn.
    /// let hand = hand.removed(PieceKind::Pawn).unwrap();
    /// assert_eq!(hand.count(PieceKind::Pawn), Some(2));
    ///
    /// // We can't discard what we don't own.
    /// let hand = hand.removed(PieceKind::Bishop);
    /// assert_eq!(hand, None);
    /// ```
    #[inline]
    pub fn removed(mut self, piece_kind: PieceKind) -> Option<Hand> {
        let index = (piece_kind as u8 - 1) as usize;
        if index < 7 {
            self.0[index] = self.0[index].checked_sub(1)?;
            return Some(self);
        }
        None
    }
    /// Finds the number of pieces `piece_kind` in `self`.
    #[inline]
    pub fn count(self, piece_kind: PieceKind) -> Option<u8> {
        let index = (piece_kind as u8 - 1) as usize;
        if index < 7 {
            return Some(self.0[index]);
        }
        None
    }

    /// C interface of [`Hand::added`].
    ///
    /// This function returns true if and only if adding was successful.
    #[no_mangle]
    pub extern "C" fn Hand_add(&mut self, piece_kind: PieceKind) -> bool {
        if let Some(new) = self.added(piece_kind) {
            *self = new;
            true
        } else {
            false
        }
    }

    /// C interface of [`Hand::removed`].
    ///
    /// This function returns true if and only if removal was successful.
    #[no_mangle]
    pub extern "C" fn Hand_remove(&mut self, piece_kind: PieceKind) -> bool {
        if let Some(new) = self.removed(piece_kind) {
            *self = new;
            true
        } else {
            false
        }
    }

    /// C interface of [`Hand::count`].
    ///
    /// This function returns true if and only if `piece_kind` can be a piece in hand.
    #[no_mangle]
    pub extern "C" fn Hand_count(self, piece_kind: PieceKind) -> u8 {
        self.count(piece_kind).unwrap_or(0)
    }

    #[inline(always)]
    fn as_u64(self) -> u64 {
        // Safety: `sizeof::<[u8; 8]>()` = `sizeof::<u64>()` = 8
        unsafe { core::mem::transmute(self) }
    }
}

impl PartialEq for Hand {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_u64() == other.as_u64()
    }
}

impl_ord_for_single_field!(Hand);
impl_hash_for_single_field!(Hand);

/// Finds the USI representation of hand: <https://web.archive.org/web/20080131070731/http://www.glaurungchess.com/shogi/usi.html>
///
/// The order of pieces are defined: `RBGSNLPrbgsnlp`.
///
/// Examples:
/// ```
/// use shogi_core::{Hand, PieceKind, ToUsi};
/// let hand = Hand::new().added(PieceKind::Pawn).unwrap();
/// let hand = hand.added(PieceKind::Pawn).unwrap();
/// let hand = hand.added(PieceKind::Pawn).unwrap();
///
/// assert_eq!([hand; 2].to_usi_owned(), "3P3p");
///
/// let hand = hand.added(PieceKind::Rook).unwrap();
/// let hand = hand.added(PieceKind::Bishop).unwrap();
/// assert_eq!([Hand::default(), hand].to_usi_owned(), "rb3p");
/// ```
impl ToUsi for [Hand; 2] {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        if self[0] == Hand::new() && self[1] == Hand::new() {
            return sink.write_str("-");
        }
        let pieces = [b"PLNSGBR", b"plnsgbr"];
        for i in 0..2 {
            for j in (0..7).rev() {
                // Safety: 0 <= j < 8
                let count = *unsafe { self[i].0.get_unchecked(j) };
                if count > 0 {
                    if count >= 2 {
                        write_u8(sink, count)?;
                    }
                    // Safety: `pieces[i][j]` is an ASCII byte.
                    // Furthermore, 0 <= j < 8 holds, which implies pieces[i][j] is always in bounds.
                    unsafe { write_ascii_byte(sink, *pieces[i].get_unchecked(j)) }?;
                }
            }
        }
        Ok(())
    }
}
