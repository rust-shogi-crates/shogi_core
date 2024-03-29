use core::fmt::{Result as FmtResult, Write};
use core::mem::MaybeUninit;

use crate::c_compat::{OptionCompactMove, OptionGameResolution, OptionPiece, OptionSquare};
use crate::common::{write_ascii_byte, write_u16, write_u8};
use crate::{
    Bitboard, Color, CompactMove, GameResolution, Hand, Move, Piece, PieceKind, Square, ToUsi,
};

/// A record of a game. A position and how a game is resolved.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct Game {
    inner: Position,
    resolution: OptionGameResolution,
}

#[cfg(feature = "alloc")]
impl Game {
    /// Returns the inner position.
    #[export_name = "Game_position"]
    #[inline(always)]
    pub extern "C" fn position(&self) -> &Position {
        &self.inner
    }
    /// Sets the resolution of this game.
    #[export_name = "Game_resolve"]
    #[inline(always)]
    pub extern "C" fn resolve(&mut self, resolution: GameResolution) {
        self.resolution = Some(resolution).into();
    }
    /// Unsets the resolution of this game.
    #[export_name = "Game_unresolve"]
    #[inline(always)]
    pub extern "C" fn unresolve(&mut self) {
        self.resolution = None.into();
    }
    /// Returns the resolution of this game.
    #[inline(always)]
    pub fn resolution(&self) -> Option<GameResolution> {
        self.resolution.into()
    }
    /// C interface to [`Game::resolution`].
    #[no_mangle]
    pub extern "C" fn Game_resolution(&self) -> OptionGameResolution {
        self.resolution
    }
}

#[cfg(feature = "alloc")]
impl_ord_with_fields!(Game; inner, resolution);
#[cfg(feature = "alloc")]
impl_hash_with_fields!(Game; inner, resolution);

/// A record of a game. A position and how a game is resolved.
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct PartialGame {
    inner: PartialPosition,
    resolution: OptionGameResolution,
}

impl PartialGame {
    /// Returns the inner position.
    #[export_name = "PartialGame_position"]
    #[inline(always)]
    pub extern "C" fn position(&self) -> &PartialPosition {
        &self.inner
    }
    /// Sets the resolution of this game.
    #[export_name = "PartialGame_resolve"]
    #[inline(always)]
    pub extern "C" fn resolve(&mut self, resolution: GameResolution) {
        self.resolution = Some(resolution).into();
    }
    /// Unsets the resolution of this game.
    #[export_name = "PartialGame_unresolve"]
    #[inline(always)]
    pub extern "C" fn unresolve(&mut self) {
        self.resolution = None.into();
    }
    /// Returns the resolution of this game.
    #[inline(always)]
    pub fn resolution(&self) -> Option<GameResolution> {
        self.resolution.into()
    }
    /// C interface to [`PartialGame::resolution`].
    #[no_mangle]
    pub extern "C" fn PartialGame_resolution(&self) -> OptionGameResolution {
        self.resolution
    }
}

impl_ord_with_fields!(PartialGame; inner, resolution);
impl_hash_with_fields!(PartialGame; inner, resolution);

/// A position. It provides sufficient data for legality checking.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct Position {
    initial: PartialPosition,
    inner: PartialPosition,
    moves: alloc::vec::Vec<Move>,
}

#[cfg(feature = "alloc")]
impl Position {
    /// Returns the inner [`PartialPosition`].
    #[export_name = "Position_inner"]
    #[inline(always)]
    pub extern "C" fn inner(&self) -> &PartialPosition {
        &self.inner
    }

    /// Returns the initial position of [`Position`], i.e., the position before any moves given to it.
    #[export_name = "Position_initial_position"]
    #[inline(always)]
    pub extern "C" fn initial_position(&self) -> &PartialPosition {
        &self.initial
    }

    /// Creates a [`Position`] with the starting position of shogi.
    pub fn startpos() -> Self {
        Self::arbitrary_position(PartialPosition::startpos())
    }

    /// C interface of [`Position::startpos`].
    #[no_mangle]
    pub extern "C" fn Position_startpos() -> *mut Self {
        alloc::boxed::Box::leak(alloc::boxed::Box::new(Self::startpos()))
    }

    /// Destructs a [`Position`].
    ///
    /// # Safety
    /// `ptr` must be the one created by a function in this type.
    #[no_mangle]
    pub unsafe extern "C" fn Position_destruct(ptr: *mut Self) {
        drop(alloc::boxed::Box::from_raw(ptr));
    }

    /// Creates a [`Position`] with its initial position `p`.
    pub fn arbitrary_position(p: PartialPosition) -> Self {
        Self {
            initial: p.clone(),
            inner: p,
            moves: alloc::vec::Vec::new(),
        }
    }

    /// Finds which player is to move.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Move, Position, Square};
    /// let mut pos = Position::startpos();
    /// assert_eq!(pos.side_to_move(), Color::Black);
    /// pos.make_move(Move::Normal { from: Square::SQ_7G, to: Square::SQ_7F, promote: false }).unwrap();
    /// assert_eq!(pos.side_to_move(), Color::White);
    /// ```
    #[export_name = "Position_side_to_move"]
    pub extern "C" fn side_to_move(&self) -> Color {
        self.inner.side_to_move()
    }

    /// Returns the [`Hand`] of a player.
    #[export_name = "Position_hand_of_a_player"]
    pub extern "C" fn hand_of_a_player(&self, color: Color) -> Hand {
        self.inner.hand_of_a_player(color)
    }

    /// Gives the reference to the hand of the specified player.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it. Exported for parsers.
    #[deprecated(
        since = "0.1.2",
        note = "This function can create inconsistent states. It will be removed in 0.2.0."
    )]
    pub fn hand_of_a_player_mut(&mut self, color: Color) -> &mut Hand {
        self.inner.hand_of_a_player_mut(color)
    }

    /// Returns how many pieces of `piece` are in hand.
    ///
    /// If `piece` is not a valid piece in hand, this method returns [`None`].
    pub fn hand(&self, piece: Piece) -> Option<u8> {
        self.inner.hand(piece)
    }

    /// Finds how many moves were made.
    #[export_name = "Position_ply"]
    pub extern "C" fn ply(&self) -> u16 {
        self.inner.ply()
    }

    /// Returns the [`Piece`] on the designated [`Square`].
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Piece, PieceKind, Position, Square};
    /// let pos = Position::startpos();
    /// let black_rook = pos.piece_at(Square::SQ_2H);
    /// assert_eq!(black_rook, Some(Piece::B_R));
    /// let vacant = pos.piece_at(Square::SQ_3H);
    /// assert_eq!(vacant, None);
    /// ```
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.inner.piece_at(square)
    }

    /// C interface to [`Position::piece_at`].
    #[no_mangle]
    pub extern "C" fn Position_piece_at(&self, square: Square) -> OptionPiece {
        self.inner.PartialPosition_piece_at(square)
    }

    /// Place a piece on a square.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it.
    #[deprecated(
        since = "0.1.2",
        note = "This function can create inconsistent states. It will be removed in 0.2.0."
    )]
    pub fn piece_set(&mut self, square: Square, piece: Option<Piece>) {
        self.inner.piece_set(square, piece)
    }

    /// Finds the subset of squares with a piece.
    ///
    /// Since: 0.1.4
    #[export_name = "Position_occupied_bitboard"]
    #[inline(always)]
    pub extern "C" fn occupied_bitboard(&self) -> Bitboard {
        self.inner.occupied_bitboard()
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
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Bitboard, Color, Piece, PieceKind, Position, Square};
    /// let pos = Position::startpos();
    /// let black_rook = pos.piece_bitboard(Piece::B_R);
    /// assert_eq!(black_rook, Bitboard::single(Square::SQ_2H));
    /// let white_rook = pos.piece_bitboard(Piece::W_R);
    /// assert_eq!(white_rook, Bitboard::single(Square::SQ_8B));
    /// ```
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

    /// Returns all moves made so far.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Position;
    /// assert_eq!(Position::startpos().moves(), []);
    /// ```
    /// Since: 0.1.2
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    /// C interface to [`Position::last_compact_move`].
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
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::Position;
    /// let pos = Position::startpos();
    /// let s = pos.to_sfen_owned();
    /// assert_eq!(
    ///     s,
    ///     "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
    /// );
    ///```
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

#[cfg(feature = "alloc")]
impl_ord_with_fields!(Position; initial, inner, moves);
#[cfg(feature = "alloc")]
impl_hash_with_fields!(Position; initial, inner, moves);

/// A position with its move sequence omitted.
///
/// This data is insufficient for complete legality checking (such as repetition checking),
/// but in most cases it suffices. If you need a complete legality checking, use `Position`.
///
/// TODO: describe exactly when a position is considered valid
#[derive(Eq, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct PartialPosition {
    side: Color,
    ply: u16,
    hands: [Hand; 2],
    board: [OptionPiece; 81],
    player_bb: [Bitboard; 2],
    piece_bb: [Bitboard; 14],
    last_move: OptionCompactMove,
    king_square: [OptionSquare; 2],
}

impl PartialPosition {
    /// Returns an empty position.
    pub fn empty() -> Self {
        Self {
            side: Color::Black,
            ply: 1,
            hands: [Default::default(); 2],
            board: [None.into(); 81],
            player_bb: [Bitboard::empty(); 2],
            piece_bb: [Bitboard::empty(); PieceKind::NUM],
            last_move: None.into(),
            king_square: [None.into(); Color::NUM],
        }
    }

    const STARTPOS_BLACK_BB: Bitboard = {
        let mut result = Bitboard::empty();
        let mut i = 0;
        while i < 9 {
            // Safety: i+1 is in range 1..=9.
            let file = unsafe { Bitboard::from_file_unchecked(i as u8 + 1, 1 << 8 | 1 << 6) };
            result = result.or(file);
            i += 1;
        }
        result = result.or(Bitboard::single(Square::SQ_2H));
        result = result.or(Bitboard::single(Square::SQ_8H));
        result
    };

    const STARTPOS_WHITE_BB: Bitboard = {
        let mut result = Bitboard::empty();
        let mut i = 0;
        while i < 9 {
            // Safety: i+1 is in range 1..=9.
            let file = unsafe { Bitboard::from_file_unchecked(i as u8 + 1, 1 << 2 | 1) };
            result = result.or(file);
            i += 1;
        }
        result = result.or(Bitboard::single(Square::SQ_2B));
        result = result.or(Bitboard::single(Square::SQ_8B));
        result
    };

    /// Returns the starting position of shogi.
    pub fn startpos() -> Self {
        // TODO stop panicking
        let mut board: [OptionPiece; 81] = [None.into(); 81];
        // Pawns
        for i in 0..9 {
            board[6 + i * 9] = Some(Piece::B_P).into();
            board[2 + i * 9] = Some(Piece::W_P).into();
        }
        // Bishop, Rook
        board[70] = Some(Piece::B_B).into();
        board[10] = Some(Piece::W_B).into();
        board[16] = Some(Piece::B_R).into();
        board[64] = Some(Piece::W_R).into();
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
        let mut piece_bb = [Bitboard::empty(); PieceKind::NUM];
        for square in Square::all() {
            if let Some(piece) =
                <Option<Piece>>::from(*unsafe { board.get_unchecked(square.array_index()) })
            {
                let piece_kind = piece.piece_kind();
                piece_bb[piece_kind.array_index()] |= square;
            }
        }
        Self {
            side: Color::Black,
            ply: 1,
            hands: [Default::default(); 2],
            board,
            player_bb: [Self::STARTPOS_BLACK_BB, Self::STARTPOS_WHITE_BB],
            piece_bb,
            last_move: None.into(),
            king_square: [Some(Square::SQ_5I).into(), Some(Square::SQ_5A).into()],
        }
    }

    /// C interface of `startpos`.
    #[no_mangle]
    pub extern "C" fn PartialPosition_startpos(buf: &mut MaybeUninit<Self>) {
        buf.write(Self::startpos());
    }

    /// Finds which player is to move.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, Move, PartialPosition, Square};
    /// let mut pos = PartialPosition::startpos();
    /// assert_eq!(pos.side_to_move(), Color::Black);
    /// pos.make_move(Move::Normal { from: Square::SQ_7G, to: Square::SQ_7F, promote: false }).unwrap();
    /// assert_eq!(pos.side_to_move(), Color::White);
    /// ```
    #[export_name = "PartialPosition_side_to_move"]
    #[inline(always)]
    pub extern "C" fn side_to_move(&self) -> Color {
        self.side
    }

    /// Sets which player is to move.
    #[inline(always)]
    pub fn side_to_move_set(&mut self, side: Color) {
        self.side = side;
    }

    /// Returns the [`Hand`] of a player.
    #[export_name = "PartialPosition_hand_of_a_player"]
    #[inline(always)]
    pub extern "C" fn hand_of_a_player(&self, color: Color) -> Hand {
        // Safety: color as usize is either 1 or 2
        *unsafe { self.hands.get_unchecked((color as u8 - 1) as usize) }
    }

    /// Gives the reference to the hand of the specified player.
    ///
    /// This function makes no guarantee about the consistency of the position.
    /// Users should have a good reason when using it. Exported for parsers.
    #[inline(always)]
    pub fn hand_of_a_player_mut(&mut self, color: Color) -> &mut Hand {
        // Safety: color as usize is either 1 or 2
        unsafe { self.hands.get_unchecked_mut((color as u8 - 1) as usize) }
    }

    /// Returns how many pieces of `piece` are in hand.
    ///
    /// If `piece` is not a valid piece in hand, this method returns [`None`].
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

    /// Returns the [`Piece`] on the designated [`Square`].
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Color, PartialPosition, Piece, PieceKind, Square};
    /// let pos = PartialPosition::startpos();
    /// let black_rook = pos.piece_at(Square::SQ_2H);
    /// assert_eq!(black_rook, Some(Piece::B_R));
    /// let vacant = pos.piece_at(Square::SQ_3H);
    /// assert_eq!(vacant, None);
    /// ```
    #[inline(always)]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        <Option<Piece>>::from(self.PartialPosition_piece_at(square))
    }

    /// C interface to [`PartialPosition::piece_at`].
    #[no_mangle]
    #[inline(always)]
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
        let old = self.piece_at(square);
        // Safety: square.index() is in range 1..=81
        *unsafe { self.board.get_unchecked_mut(index as usize) } = OptionPiece::from(piece);
        let single = Bitboard::single_inlined(square);
        self.player_bb[0] = single.andnot(self.player_bb[0]);
        self.player_bb[1] = single.andnot(self.player_bb[1]);
        if let Some((_, color)) = piece.map(Piece::to_parts) {
            match color {
                Color::Black => self.player_bb[0] |= single,
                Color::White => self.player_bb[1] |= single,
            }
        }
        if let Some(piece) = old {
            let piece_kind = piece.piece_kind();
            self.piece_bb[piece_kind.array_index()] =
                single.andnot(self.piece_bb[piece_kind.array_index()]);
        }
        if let Some(piece) = piece {
            let piece_kind = piece.piece_kind();
            self.piece_bb[piece_kind.array_index()] |= single;
            if let Piece::B_K = piece {
                self.king_square[0] = OptionSquare::from(Some(square));
            }
            if let Piece::W_K = piece {
                self.king_square[1] = OptionSquare::from(Some(square));
            }
        }
    }

    /// Finds the subset of squares with a piece.
    ///
    /// Since: 0.1.4
    #[export_name = "PartialPosition_occupied_bitboard"]
    #[inline(always)]
    pub extern "C" fn occupied_bitboard(&self) -> Bitboard {
        self.player_bb[0] | self.player_bb[1]
    }

    /// Finds the subset of squares with no pieces.
    #[export_name = "PartialPosition_vacant_bitboard"]
    #[inline(always)]
    pub extern "C" fn vacant_bitboard(&self) -> Bitboard {
        !self.occupied_bitboard()
    }

    /// Finds the subset of squares where a piece of the specified player is placed.
    #[export_name = "PartialPosition_player_bitboard"]
    #[inline(always)]
    pub extern "C" fn player_bitboard(&self, color: Color) -> Bitboard {
        self.player_bb[color.array_index()]
    }

    /// Finds the subset of squares where a piece is placed.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Bitboard, Color, PartialPosition, Piece, PieceKind, Square};
    /// let pos = PartialPosition::startpos();
    /// let black_rook = pos.piece_bitboard(Piece::B_R);
    /// assert_eq!(black_rook, Bitboard::single(Square::SQ_2H));
    /// let white_rook = pos.piece_bitboard(Piece::W_R);
    /// assert_eq!(white_rook, Bitboard::single(Square::SQ_8B));
    /// ```
    #[export_name = "PartialPosition_piece_bitboard"]
    #[inline(always)]
    pub extern "C" fn piece_bitboard(&self, piece: Piece) -> Bitboard {
        let (piece_kind, color) = piece.to_parts();
        self.piece_bb[piece_kind.array_index()] & self.player_bb[color.array_index()]
    }

    /// Finds the subset of squares where a [`PieceKind`] is placed.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::{Bitboard, Color, PartialPosition, PieceKind, Square};
    /// let pos = PartialPosition::startpos();
    /// let rooks = pos.piece_kind_bitboard(PieceKind::Rook);
    /// assert_eq!(rooks, Bitboard::single(Square::SQ_2H) | Bitboard::single(Square::SQ_8B));
    /// ```
    #[export_name = "PartialPosition_piece_kind_bitboard"]
    #[inline(always)]
    pub extern "C" fn piece_kind_bitboard(&self, piece_kind: PieceKind) -> Bitboard {
        self.piece_bb[piece_kind.array_index()]
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

    /// C interface to [`PartialPosition::last_compact_move`].
    #[no_mangle]
    pub extern "C" fn PartialPosition_last_compact_move(&self) -> OptionCompactMove {
        self.last_move
    }

    #[inline(always)]
    pub fn king_position(&self, color: Color) -> Option<Square> {
        self.king_square[color.array_index()].into()
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
                        write_u8(sink, vacant)?;
                        vacant = 0;
                    }
                    occupying.to_usi(sink)?;
                } else {
                    vacant += 1;
                }
            }
            if vacant > 0 {
                write_u8(sink, vacant)?;
            }
            if i < 8 {
                // Safety: '/' is in ASCII
                unsafe { write_ascii_byte(sink, b'/') }?;
            }
        }
        // Safety: ' ' is in ASCII
        unsafe { write_ascii_byte(sink, b' ') }?;
        self.side.to_usi(sink)?;
        // Safety: ' ' is in ASCII
        unsafe { write_ascii_byte(sink, b' ') }?;
        self.hands.to_usi(sink)?;
        // Safety: ' ' is in ASCII
        unsafe { write_ascii_byte(sink, b' ') }?;
        write_u16(sink, self.ply)?;
        Ok(())
    }

    /// Returns the SFEN representation of the current position.
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::PartialPosition;
    /// let pos = PartialPosition::startpos();
    /// let s = pos.to_sfen_owned();
    /// assert_eq!(
    ///     s,
    ///     "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
    /// );
    ///```
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
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
            #[inline(always)]
            fn write_str(&mut self, s: &str) -> FmtResult {
                fn inner(this: &mut Bridge, s: &str) {
                    let slice = s.as_bytes();
                    unsafe {
                        for (i, &byte) in slice.iter().enumerate() {
                            core::ptr::write(this.0.add(i), byte);
                        }
                        this.0 = this.0.add(slice.len());
                    }
                }
                inner(self, s);
                Ok(())
            }
        }
        let mut sink = Bridge(ptr);
        let _ = self.to_sfen(&mut sink);
        // Safety: nul is in ASCII
        let _ = write_ascii_byte(&mut sink, b'\0');
    }
}

impl_ord_with_fields!(PartialPosition; side, ply, hands, board, last_move);
impl_hash_with_fields!(PartialPosition; side, ply, hands, board, last_move);

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
            from: Square::SQ_7G,
            to: Square::SQ_7F,
            promote: false,
        }); // 7g7f
        assert_eq!(result, Some(()));
        assert_eq!(s.side_to_move(), Color::White);
        assert_eq!(
            s.to_sfen_owned(),
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 2",
        );

        s.make_move(Move::Normal {
            from: Square::SQ_3C,
            to: Square::SQ_3D,
            promote: false,
        })
        .unwrap(); // 3c3d
        s.make_move(Move::Normal {
            from: Square::SQ_8H,
            to: Square::SQ_2B,
            promote: true,
        })
        .unwrap(); // 8h2b+
        s.make_move(Move::Normal {
            from: Square::SQ_3A,
            to: Square::SQ_2B,
            promote: false,
        })
        .unwrap(); // 3a2b
        assert_eq!(s.hand(Piece::B_B), Some(1));
        assert_eq!(s.hand(Piece::W_B), Some(1));
        assert_eq!(
            s.to_sfen_owned(),
            "lnsgkg1nl/1r5s1/pppppp1pp/6p2/9/2P6/PP1PPPPPP/7R1/LNSGKGSNL b Bb 5",
        );
    }
    #[test]
    fn to_sfen_works() {
        let pos = PartialPosition::startpos();
        let s = pos.to_sfen_owned();
        assert_eq!(
            s,
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
        );
    }
}
