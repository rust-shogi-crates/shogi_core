use crate::{PieceKind, ToUsi};

/// A hand of a single player. A hand is a multiset of unpromoted pieces (except a king).
///
/// This type can hold up to 255 pieces of each kind, although the rule of shogi prohibits it.
///
/// Because `Hand` is cheap to copy, it implements [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
/// Its [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html) value is an empty instance.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct Hand([u8; 8]);

impl Hand {
    /// Creates an empty instance of `Hand`.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Hand;
    /// assert_eq!(Hand::new(), Hand::default());
    /// ```
    #[export_name = "Hand_new"]
    pub extern "C" fn new() -> Self {
        Self::default()
    }

    /// Find a new `Hand`, with `piece_kind` added.
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

    /// Find a new `Hand`, with a single piece of `piece_kind` removed.
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

    /// C interface of `Hand::added`.
    #[no_mangle]
    pub extern "C" fn Hand_add(&mut self, piece_kind: PieceKind) -> bool {
        if let Some(new) = self.added(piece_kind) {
            *self = new;
            true
        } else {
            false
        }
    }

    /// C interface of `Hand::removed`.
    #[no_mangle]
    pub extern "C" fn Hand_remove(&mut self, piece_kind: PieceKind) -> bool {
        if let Some(new) = self.removed(piece_kind) {
            *self = new;
            true
        } else {
            false
        }
    }

    /// C interface of `Hand::count`.
    #[no_mangle]
    pub extern "C" fn Hand_count(self, piece_kind: PieceKind) -> u8 {
        self.count(piece_kind).unwrap_or(0)
    }
}

impl ToUsi for [Hand; 2] {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        if self[0].0.iter().all(|&x| x == 0) && self[1].0.iter().all(|&x| x == 0) {
            return sink.write_char('-');
        }
        let pieces = [
            ['P', 'L', 'N', 'S', 'G', 'B', 'R'],
            ['p', 'l', 'n', 's', 'g', 'b', 'r'],
        ];
        for i in 0..2 {
            for j in 0..7 {
                if self[i].0[j] > 0 {
                    if self[i].0[j] >= 2 {
                        sink.write_fmt(format_args!("{}", self[i].0[j]))?;
                    }
                    sink.write_char(pieces[i][j])?;
                }
            }
        }
        Ok(())
    }
}
