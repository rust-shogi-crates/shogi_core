use core::fmt::{Result as FmtResult, Write};
use core::mem::MaybeUninit;

use crate::c_compat::{OptionCompactMove, OptionPiece};
use crate::common::write_u16;
use crate::{Bitboard, Color, CompactMove, Hand, Move, Piece, PieceKind, Square, ToUsi};

/// A position. It provides sufficient data for legality checking.
#[cfg(feature = "alloc")]
#[derive(Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct Position {
    initial: PartialPosition,
    inner: PartialPosition,
    moves: alloc::vec::Vec<Move>,
}

#[cfg(feature = "alloc")]
impl Position {
    /// Returns the inner `PartialPosition`.
    #[export_name = "Position_inner"]
    pub extern "C" fn inner(&self) -> &PartialPosition {
        &self.inner
    }

    /// Returns the initial position of [Position], i.e., the position before any moves given to it.
    #[export_name = "Position_initial_position"]
    pub extern "C" fn initial_position(&self) -> &PartialPosition {
        &self.initial
    }

    pub fn startpos() -> Self {
        Self::arbitrary_position(PartialPosition::startpos())
    }

    /// C interface of `startpos`.
    #[no_mangle]
    pub extern "C" fn Position_startpos() -> *mut Self {
        alloc::boxed::Box::leak(alloc::boxed::Box::new(Self::startpos()))
    }

    /// Destructs a `Position`.
    ///
    /// # Safety
    /// `ptr` must be the one created by a function in this type.
    #[no_mangle]
    pub unsafe extern "C" fn Position_destruct(ptr: *mut Self) {
        drop(alloc::boxed::Box::from_raw(ptr));
    }

    pub fn arbitrary_position(p: PartialPosition) -> Self {
        Self {
            initial: p.clone(),
            inner: p,
            moves: alloc::vec::Vec::new(),
        }
    }

    /// Finds which player is to move.
    #[export_name = "Position_side_to_move"]
    pub extern "C" fn side_to_move(&self) -> Color {
        self.inner.side_to_move()
    }

    #[export_name = "Position_hand_of_a_player"]
    pub extern "C" fn hand_of_a_player(&self, color: Color) -> Hand {
        self.inner.hand_of_a_player(color)
    }

    /// Gives the reference to the hand of the specified player.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it. Exported for parsers.
    pub fn hand_of_a_player_mut(&mut self, color: Color) -> &mut Hand {
        self.inner.hand_of_a_player_mut(color)
    }

    pub fn hand(&self, piece: Piece) -> Option<u8> {
        self.inner.hand(piece)
    }

    /// Finds how many moves were made.
    #[export_name = "Position_ply"]
    pub extern "C" fn ply(&self) -> u16 {
        self.inner.ply()
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.inner.piece_at(square)
    }

    #[no_mangle]
    pub extern "C" fn Position_piece_at(&self, square: Square) -> OptionPiece {
        self.inner.PartialPosition_piece_at(square)
    }

    /// Place a piece on a square.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it.
    pub fn piece_set(&mut self, square: Square, piece: Option<Piece>) {
        self.inner.piece_set(square, piece)
    }

    /// Finds the subset of squares with no pieces.
    #[export_name = "Position_vacant_bitboard"]
    pub extern "C" fn vacant_bitboard(&self) -> Bitboard {
        self.inner.vacant_bitboard()
    }

    /// Finds the subset of squares where a piece of the specified player is placed.
    #[export_name = "Position_player_bitboard"]
    pub extern "C" fn player_bitboard(&self, color: Color) -> Bitboard {
        self.inner.player_bitboard(color)
    }

    /// Finds the subset of squares where a piece is placed.
    #[export_name = "Position_piece_bitboard"]
    pub extern "C" fn piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.inner.piece_bitboard(piece)
    }

    /// Returns the last move, if it exists.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Position;
    /// assert_eq!(Position::startpos().last_move(), None);
    /// ```
    pub fn last_move(&self) -> Option<Move> {
        self.inner.last_move()
    }

    /// Returns the last move, if it exists.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Position;
    /// assert_eq!(Position::startpos().last_compact_move(), None);
    /// ```
    pub fn last_compact_move(&self) -> Option<CompactMove> {
        self.inner.last_compact_move()
    }

    #[no_mangle]
    pub extern "C" fn Position_last_compact_move(&self) -> OptionCompactMove {
        self.inner.PartialPosition_last_compact_move()
    }

    /// Makes a move. Note that this function will never check legality.
    ///
    /// Returns Some(()) if the given move makes sense, i.e.,
    /// moves a piece to another square or drops a piece on a vacant square.
    ///
    /// If it returns None, it is guaranteed that self is not modified.
    #[must_use]
    pub fn make_move(&mut self, mv: Move) -> Option<()> {
        self.inner.make_move(mv)?;
        self.moves.push(mv);
        Some(())
    }

    /// Makes a move. This function is a C-compatible counterpart of `make_move`.
    /// Note that this function will never check legality.
    ///
    /// Returns true if the given move makes sense, i.e.,
    /// moves a piece to another square or drops a piece on a vacant square.
    ///
    /// If it returns false, it is guaranteed that self is not modified.
    #[export_name = "Position_make_compact_move"]
    pub extern "C" fn make_compact_move(&mut self, mv: CompactMove) -> bool {
        let mv = mv.into();
        self.make_move(mv).is_some()
    }

    // TODO: fn revert_move(&mut self) -> Option<Move>

    /// Returns the SFEN representation of the current position.
    pub fn to_sfen_owned(&self) -> alloc::string::String {
        self.inner.to_sfen_owned()
    }

    /// C interface of `to_sfen`.
    ///
    /// # Safety
    /// This function writes to `ptr` at most 139 (= 129 + 1 + 1 + 1 + 0 + 1 + 5 + 1) bytes.
    /// Caller should ensure that `ptr` has enough space for that.
    #[export_name = "Position_to_sfen_c"]
    pub unsafe extern "C" fn to_sfen_c(&self, ptr: *mut u8) {
        self.inner.to_sfen_c(ptr)
    }
}

/// A position with its move sequence omitted.
///
/// This data is insufficient for complete legality checking (such as repetition checking),
/// but in most cases it suffices. If you need a complete legality checking, use `Position`.
///
/// TODO: describe exactly when a position is considered valid
#[derive(Eq, PartialEq, Clone, Debug)]
#[repr(C)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct PartialPosition {
    side: Color,
    ply: u16,
    hands: [Hand; 2],
    board: [OptionPiece; 81],
    last_move: OptionCompactMove,
}

impl PartialPosition {
    /// Returns an empty position.
    pub fn empty() -> Self {
        Self {
            side: Color::Black,
            ply: 1,
            hands: [Default::default(); 2],
            board: [None.into(); 81],
            last_move: None.into(),
        }
    }

    /// Returns the starting position of shogi.
    pub fn startpos() -> Self {
        // TODO stop panicking
        let mut board = [None.into(); 81];
        // Pawns
        for i in 0..9 {
            board[6 + i * 9] = Some(Piece::new(PieceKind::Pawn, Color::Black)).into();
            board[2 + i * 9] = Some(Piece::new(PieceKind::Pawn, Color::White)).into();
        }
        // Bishop, Rook
        board[70] = Some(Piece::new(PieceKind::Bishop, Color::Black)).into();
        board[10] = Some(Piece::new(PieceKind::Bishop, Color::White)).into();
        board[16] = Some(Piece::new(PieceKind::Rook, Color::Black)).into();
        board[64] = Some(Piece::new(PieceKind::Rook, Color::White)).into();
        // Other minor pieces
        let order = [
            PieceKind::Lance,
            PieceKind::Knight,
            PieceKind::Silver,
            PieceKind::Gold,
            PieceKind::King,
            PieceKind::Gold,
            PieceKind::Silver,
            PieceKind::Knight,
            PieceKind::Lance,
        ];
        for i in 0..9 {
            board[8 + 9 * i] = Some(Piece::new(order[i], Color::Black)).into();
            board[9 * i] = Some(Piece::new(order[i], Color::White)).into();
        }
        Self {
            side: Color::Black,
            ply: 1,
            hands: [Default::default(); 2],
            board,
            last_move: None.into(),
        }
    }

    /// C interface of `startpos`.
    #[no_mangle]
    pub extern "C" fn PartialPosition_startpos(buf: &mut MaybeUninit<Self>) {
        buf.write(Self::startpos());
    }

    /// Finds which player is to move.
    #[export_name = "PartialPosition_side_to_move"]
    pub extern "C" fn side_to_move(&self) -> Color {
        self.side
    }

    /// Sets which player is to move.
    pub fn side_to_move_set(&mut self, side: Color) {
        self.side = side;
    }

    #[export_name = "PartialPosition_hand_of_a_player"]
    pub extern "C" fn hand_of_a_player(&self, color: Color) -> Hand {
        // Safety: color as usize is either 1 or 2
        *unsafe { self.hands.get_unchecked((color as u8 - 1) as usize) }
    }

    /// Gives the reference to the hand of the specified player.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it. Exported for parsers.
    pub fn hand_of_a_player_mut(&mut self, color: Color) -> &mut Hand {
        // Safety: color as usize is either 1 or 2
        unsafe { self.hands.get_unchecked_mut((color as u8 - 1) as usize) }
    }

    pub fn hand(&self, piece: Piece) -> Option<u8> {
        let hand = self.hand_of_a_player(piece.color());
        hand.count(piece.piece_kind())
    }

    /// Finds how many moves were made.
    #[export_name = "PartialPosition_ply"]
    #[must_use]
    pub extern "C" fn ply(&self) -> u16 {
        self.ply
    }

    /// Sets how many moves are made. Returns whether this operation was successful.
    /// This operation succeeds iff `ply != 0`.
    #[must_use]
    pub fn ply_set(&mut self, ply: u16) -> bool {
        if ply == 0 {
            return false;
        }
        self.ply = ply;
        true
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.PartialPosition_piece_at(square).into()
    }

    #[no_mangle]
    pub extern "C" fn PartialPosition_piece_at(&self, square: Square) -> OptionPiece {
        let index = square.index() - 1;
        // Safety: square.index() is in range 1..=81
        *unsafe { self.board.get_unchecked(index as usize) }
    }

    /// Place a piece on a square.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it. Exported for parsers.
    pub fn piece_set(&mut self, square: Square, piece: Option<Piece>) {
        let index = square.index() - 1;
        // Safety: square.index() is in range 1..=81
        *unsafe { self.board.get_unchecked_mut(index as usize) } = piece.into();
    }

    /// Finds the subset of squares with no pieces.
    #[export_name = "PartialPosition_vacant_bitboard"]
    pub extern "C" fn vacant_bitboard(&self) -> Bitboard {
        // TODO: optimize to allow O(1)-time retrieval
        let mut result = Bitboard::empty();
        for i in 0..81 {
            if Option::<Piece>::from(self.board[i]).is_none() {
                let square = unsafe { Square::from_u8_unchecked(i as u8 + 1) };
                result |= Bitboard::single(square);
            }
        }
        result
    }

    /// Finds the subset of squares where a piece of the specified player is placed.
    #[export_name = "PartialPosition_player_bitboard"]
    pub extern "C" fn player_bitboard(&self, color: Color) -> Bitboard {
        // TODO: optimize to allow O(1)-time retrieval
        let mut result = Bitboard::empty();
        for i in 0..81 {
            if let Some(piece) = Option::<Piece>::from(self.board[i]) {
                if piece.color() == color {
                    let square = unsafe { Square::from_u8_unchecked(i as u8 + 1) };
                    result |= Bitboard::single(square);
                }
            }
        }
        result
    }

    /// Finds the subset of squares where a piece is placed.
    #[export_name = "PartialPosition_piece_bitboard"]
    pub extern "C" fn piece_bitboard(&self, piece: Piece) -> Bitboard {
        // TODO: optimize to allow O(1)-time retrieval
        let mut result = Bitboard::empty();
        for i in 0..81 {
            if self.board[i] == Some(piece).into() {
                let square = unsafe { Square::from_u8_unchecked(i as u8 + 1) };
                result |= Bitboard::single(square);
            }
        }
        result
    }

    /// Returns the last move, if it exists.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::PartialPosition;
    /// assert_eq!(PartialPosition::startpos().last_move(), None);
    /// ```
    pub fn last_move(&self) -> Option<Move> {
        self.last_compact_move().map(|mv| mv.into())
    }

    /// Returns the last move, if it exists.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::PartialPosition;
    /// assert_eq!(PartialPosition::startpos().last_compact_move(), None);
    /// ```
    pub fn last_compact_move(&self) -> Option<CompactMove> {
        self.last_move.into()
    }

    #[no_mangle]
    pub extern "C" fn PartialPosition_last_compact_move(&self) -> OptionCompactMove {
        self.last_move
    }

    /// Makes a move. Note that this function will never check legality.
    ///
    /// Returns Some(()) if the given move makes sense, i.e.,
    /// moves a piece to another square or drops a piece on a vacant square.
    ///
    /// If it returns None, it is guaranteed that self is not modified.
    pub fn make_move(&mut self, mv: Move) -> Option<()> {
        let color = self.side;
        match mv {
            Move::Normal { from, to, promote } => {
                let piece = self.piece_at(from)?;
                if piece.color() != color {
                    return None;
                }
                let target_piece = if promote { piece.promote()? } else { piece };
                if let Some(enemy) = self.piece_at(to) {
                    if piece.color() == enemy.color() {
                        return None;
                    }
                    let hand = self.hand_of_a_player_mut(piece.color());
                    let obtaining = enemy.piece_kind();
                    let unpromoted = if let Some(piece_kind) = obtaining.unpromote() {
                        piece_kind
                    } else {
                        obtaining
                    };
                    *hand = hand.added(unpromoted)?;
                }
                self.piece_set(from, None);
                self.piece_set(to, Some(target_piece));
            }
            Move::Drop { piece, to } => {
                if piece.color() != color {
                    return None;
                }
                if piece.unpromote().is_some() {
                    return None;
                }
                if self.piece_at(to).is_some() {
                    return None;
                }
                let hand = self.hand_of_a_player_mut(piece.color());
                *hand = hand.removed(piece.piece_kind())?;
                self.piece_set(to, Some(piece));
            }
        }
        self.last_move = Some(mv.into()).into();
        self.side = self.side.flip();
        self.ply = self.ply.wrapping_add(1);
        Some(())
    }

    /// Makes a move. This function is a C-compatible counterpart of `make_move`.
    /// Note that this function will never check legality.
    ///
    /// Returns true if the given move makes sense, i.e.,
    /// moves a piece to another square or drops a piece on a vacant square.
    ///
    /// If it returns false, it is guaranteed that self is not modified.
    #[export_name = "PartialPosition_make_compact_move"]
    pub extern "C" fn make_compact_move(&mut self, mv: CompactMove) -> bool {
        let mv = mv.into();
        self.make_move(mv).is_some()
    }

    /// Write the current position in SFEN notation.
    pub fn to_sfen<W: Write>(&self, sink: &mut W) -> FmtResult {
        for i in 0..9 {
            let mut vacant = 0;
            for j in 0..9 {
                // Safety: the index is in range 0..81.
                let current = *unsafe { self.board.get_unchecked(9 * (8 - j) + i) };
                let current: Option<Piece> = current.into();
                if let Some(occupying) = current {
                    if vacant > 0 {
                        sink.write_char((b'0' + vacant as u8) as char)?;
                        vacant = 0;
                    }
                    occupying.to_usi(sink)?;
                } else {
                    vacant += 1;
                }
            }
            if vacant > 0 {
                sink.write_char((b'0' + vacant as u8) as char)?;
            }
            if i < 8 {
                sink.write_char('/')?;
            }
        }
        sink.write_char(' ')?;
        self.side.to_usi(sink)?;
        sink.write_char(' ')?;
        self.hands.to_usi(sink)?;
        sink.write_char(' ')?;
        write_u16(sink, self.ply)?;
        Ok(())
    }

    /// Returns the SFEN representation of the current position.
    #[cfg(feature = "alloc")]
    pub fn to_sfen_owned(&self) -> alloc::string::String {
        let mut s = alloc::string::String::new();
        let _ = self.to_sfen(&mut s); // Cannot fail
        s
    }

    /// C interface of `to_sfen`.
    ///
    /// # Safety
    /// This function writes to `ptr` at most 139 (= 129 + 1 + 1 + 1 + 0 + 1 + 5 + 1) bytes.
    /// Caller should ensure that `ptr` has enough space for that.
    #[export_name = "PartialPosition_to_sfen_c"]
    pub unsafe extern "C" fn to_sfen_c(&self, ptr: *mut u8) {
        struct Bridge(*mut u8);
        impl Write for Bridge {
            fn write_str(&mut self, s: &str) -> FmtResult {
                let slice = s.as_bytes();
                unsafe {
                    for (i, &byte) in slice.iter().enumerate() {
                        core::ptr::write(self.0.add(i), byte);
                    }
                    self.0 = self.0.add(slice.len());
                }
                Ok(())
            }
        }
        let mut sink = Bridge(ptr);
        let _ = self.to_sfen(&mut sink);
        let _ = sink.write_char('\0');
    }
}

impl Default for PartialPosition {
    fn default() -> Self {
        Self::startpos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos_works() {
        let mut s = PartialPosition::startpos();
        let result = s.make_move(Move::Normal {
            from: Square::new(7, 7).unwrap(),
            to: Square::new(7, 6).unwrap(),
            promote: false,
        }); // 7g7f
        assert_eq!(result, Some(()));
        assert_eq!(s.side_to_move(), Color::White);

        s.make_move(Move::Normal {
            from: Square::new(3, 3).unwrap(),
            to: Square::new(3, 4).unwrap(),
            promote: false,
        })
        .unwrap(); // 3c3d
        s.make_move(Move::Normal {
            from: Square::new(8, 8).unwrap(),
            to: Square::new(2, 2).unwrap(),
            promote: true,
        })
        .unwrap(); // 8h2b+
        s.make_move(Move::Normal {
            from: Square::new(3, 1).unwrap(),
            to: Square::new(2, 2).unwrap(),
            promote: false,
        })
        .unwrap(); // 3a2b
        assert_eq!(s.hand(Piece::new(PieceKind::Bishop, Color::Black)), Some(1));
        assert_eq!(s.hand(Piece::new(PieceKind::Bishop, Color::White)), Some(1));
        assert_eq!(
            s.to_sfen_owned(),
            "lnsgkg1nl/1r5s1/pppppp1pp/6p2/9/2P6/PP1PPPPPP/7R1/LNSGKGSNL B Bb 5",
        );
    }
    #[test]
    fn to_sfen_works() {
        let pos = PartialPosition::startpos();
        let s = pos.to_sfen_owned();
        assert_eq!(
            s,
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL B - 1",
        );
    }
}
