#ifndef shogi_core_bindings_h
#define shogi_core_bindings_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


/**
 * A player.
 *
 * [`Color`] and <code>[Option]<[Color]></code> are both 1-byte data types.
 * Because they are cheap to copy, they implement [`Copy`].
 */
enum Color {
  /**
   * Black, who plays first. Known as `先手` (*sente*).
   *
   * Its representation is 1.
   */
  Black = 1,
  /**
   * White, who plays second. Known as `後手` (*gote*).
   *
   * Its representation is 2.
   */
  White = 2,
};
typedef uint8_t Color;

/**
 * Kinds of pieces.
 *
 * [`PieceKind`] and <code>[Option]<[PieceKind]></code> are both 1-byte data types.
 * Because they are cheap to copy, they implement [`Copy`].
 */
enum PieceKind {
  /**
   * A pawn. Unlike in chess, it always moves one square forward,
   * even if the destination square is occuipied by an enemy piece.
   *
   * Known as `歩` (*fu*) or `歩兵` (*fuhyō*), although the latter name is rarely used.
   *
   * Discriminant = 1.
   */
  Pawn = 1,
  /**
   * A lance. It moves any number of squares forward without jumping over other pieces.
   * Chess has no counterpart of it.
   *
   * Known as `香` (*kyō*) or `香車` (*kyōsha*).
   *
   * Discriminant = 2.
   */
  Lance = 2,
  /**
   * A knight. Unlike in chess, it can only move two squares forward and one square vertically.
   *
   * Known as `桂` (*kē*) or `桂馬` (*kēma*).
   *
   * Discriminant = 3.
   */
  Knight = 3,
  /**
   * A silver general. It moves one square forward or diagonally.
   * Chess has no counterpart of it.
   *
   * Known as `銀` (*gin*) or `銀将` (*ginshō*), although the latter name is rarely used.
   *
   * Discriminant = 4.
   */
  Silver = 4,
  /**
   * A gold general. It moves one square horizontally, vertically, and diagonally forward.
   * Chess has no counterpart of it.
   *
   * Known as `金` (*kin*) or `金将` (*kinshō*), although the latter name is rarely used.
   *
   * Discriminant = 5.
   */
  Gold = 5,
  /**
   * A bishop. It moves any number of squares diagonally,
   * exactly the same way as a bishop does in chess.
   *
   * Known as `角` (*kaku*) or `角行` (*kakugyō*), although the latter name is rarely used.
   *
   * Discriminant = 6.
   */
  Bishop = 6,
  /**
   * A rook. It moves any number of squares horizontally or vertically.
   * It is almost the same as a rook in chess, but shogi has no rule of castling.
   *
   * Known as `飛` (*hi*) or `飛車` (*hisha*), although the former name is rarely used to refer to a piece.
   *
   * Discriminant = 7.
   */
  Rook = 7,
  /**
   * A king. It moves one square horizontally, vertically or diagonally.
   * A move that would expose the king to an enemy piece's capture threat is an illegal move,
   * and the player that has no legal moves immediately loses.
   *
   * It is almost the same as a king in chess, but shogi has no rule of castling.
   *
   * Known as `王` (*ō*), `王将` (*ōshō*), `玉` (*gyoku*) or `玉将` (*gyokushō*).
   * The two-letter names are rarely used to refer to pieces.
   *
   * Discriminant = 8.
   */
  King = 8,
  /**
   * A promoted pawn. Moves exactly the same way as a gold general.
   *
   * Known as `と` (*to*) or `と金` (*tokin*),
   * although the former name is rarely used to refer to a piece.
   *
   * Discriminant = 9.
   */
  ProPawn = 9,
  /**
   * A promoted lance. Moves exactly the same way as a gold general.
   *
   * Known as `成香` (*narikyō*).
   *
   * Discriminant = 10.
   */
  ProLance = 10,
  /**
   * A promoted knight. Moves exactly the same way as a gold general.
   *
   * Known as `成桂` (*narikē*).
   *
   * Discriminant = 11.
   */
  ProKnight = 11,
  /**
   * A promoted silver general. Moves exactly the same way as a gold general.
   *
   * Known as `成銀` (*narigin*).
   *
   * Discriminant = 12.
   */
  ProSilver = 12,
  /**
   * A promoted bishop. It moves any number of squares diagonally, or one square horizontally or vertically.
   *
   * Known as `馬` (*uma*), `竜馬` (*ryūma*),
   * although the latter is rarely used and confusing.
   *
   * Discriminant = 13.
   */
  ProBishop = 13,
  /**
   * A promoted rook.  It moves any number of squares horizontally or vertically, or one square diagonally.
   *
   * Known as `竜` (*ryū*), `竜王` (*ryūō*),
   * although the latter is rarely used and confusing.
   *
   * Discriminant = 14.
   */
  ProRook = 14,
};
typedef uint8_t PieceKind;

typedef struct Option_Piece Option_Piece;

typedef struct Option_Square Option_Square;

#if defined(DEFINE_ALLOC)
/**
 * A position. It provides sufficient data for legality checking.
 */
typedef struct Position Position;
#endif

/**
 * A subset of all squares.
 *
 * Because [`Bitboard`] is cheap to copy, it implements [`Copy`].
 * Its [`Default`] value is an empty instance.
 */
typedef struct Bitboard {
  uint64_t _0[2];
} Bitboard;

/**
 * A square.
 *
 * [`Square`] and <code>[Option]<[Square]></code> are both 1-byte data types.
 * Because they are cheap to copy, they implement [`Copy`].
 */
typedef uint8_t Square;

/**
 * C interface of <code>[Option]<[Square]></code>.
 *
 * This type is provided for C interoperability.
 * cbindgen cannot deduce that <code>[Option]<[Square]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
 * Users of this type should convert to/from <code>[Option]<[Square]></code>.
 *
 * See: <https://github.com/eqrion/cbindgen/issues/326>.
 */
typedef uint8_t OptionSquare;

/**
 * A move packed in two bytes. C-compatible version of [`Move`].
 *
 * Representation is as follows:
 * - normal move: promote * 32768 + from * 256 + to
 * - drop move: piece * 256 + 128 + to
 *
 * Note that the representation cannot be zero.
 */
typedef uint16_t CompactMove;

/**
 * A piece + who owns it.
 *
 * `Piece` and `Option<Piece>` are both 1-byte data types.
 * Because they are cheap to copy, they implement [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
 *
 * Examples:
 * ```
 * use shogi_core::Piece;
 * assert_eq!(core::mem::size_of::<Piece>(), 1);
 * ```
 */
typedef uint8_t Piece;

/**
 * A hand of a single player. A hand is a multiset of unpromoted pieces (except a king).
 *
 * This type can hold up to 255 pieces of each kind, although the rule of shogi prohibits it.
 *
 * Because [`Hand`] is cheap to copy, it implements [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html).
 * Its [`Default`] value is an empty instance.
 */
typedef struct Hand {
  uint8_t _0[8];
} Hand;

/**
 * <https://github.com/eqrion/cbindgen/issues/326>.
 */
typedef uint8_t OptionPiece;

/**
 * C-compatible type for <code>[Option]<[CompactMove]></code>.
 *
 * cbindgen cannot deduce that <code>[Option]<[CompactMove]></code> can be represented by `uint16_t` in C, so we need to define the bridge type.
 * See: <https://github.com/eqrion/cbindgen/issues/326>.
 */
typedef uint16_t OptionCompactMove;

/**
 * A position with its move sequence omitted.
 *
 * This data is insufficient for complete legality checking (such as repetition checking),
 * but in most cases it suffices. If you need a complete legality checking, use `Position`.
 *
 * TODO: describe exactly when a position is considered valid
 */
typedef struct PartialPosition {
  Color side;
  uint16_t ply;
  struct Hand hands[2];
  OptionPiece board[81];
  OptionCompactMove last_move;
} PartialPosition;

/**
 * <code>[Option]<[PieceKind]></code> with defined representation.
 *
 * The correspondence is:
 * [`None`] => `0`, <code>[Some]\(x\)</code> => `x`.
 * Therefore, valid representations of this type are precisely `0..=14`.
 *
 * This type is provided for C interoperability.
 * cbindgen cannot deduce that <code>[Option]<[PieceKind]></code> can be represented by `uint16_t` in C, so we need to define the bridge type.
 * Users of this type should convert to/from <code>[Option]<[PieceKind]></code>.
 *
 * See: <https://github.com/eqrion/cbindgen/issues/326>.
 */
typedef uint8_t OptionPieceKind;

/**
 * Finds if `self` as a subset contains a [`Square`].
 *
 * Examples:
 * ```
 * use shogi_core::{Bitboard, Square};
 * let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
 * assert!(sq11.contains(Square::new(1, 1).unwrap()));
 * assert!(!sq11.contains(Square::new(9, 9).unwrap()));
 * ```
 */
bool Bitboard_contains(struct Bitboard self, Square square);

/**
 * Finds how many elements this [`Bitboard`] has.
 *
 * Examples:
 * ```
 * use shogi_core::{Bitboard, Square};
 * let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
 * let sq55 = Bitboard::single(Square::new(5, 5).unwrap());
 * assert_eq!((sq11 | sq55).count(), 2);
 * ```
 */
uint8_t Bitboard_count(struct Bitboard self);

/**
 * Creates an empty [`Bitboard`].
 *
 * Examples:
 * ```
 * use shogi_core::Bitboard;
 * let empty = Bitboard::empty();
 * assert_eq!(empty.count(), 0);
 * ```
 */
struct Bitboard Bitboard_empty(void);

/**
 * Finds the flipped version of `self`.
 *
 * Examples:
 * ```
 * use shogi_core::{Bitboard, Square};
 * let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
 * let sq99 = Bitboard::single(Square::new(9, 9).unwrap());
 * assert_eq!(sq11.flip(), sq99);
 * ```
 */
struct Bitboard Bitboard_flip(struct Bitboard self);

/**
 * Checks if `self` is an empty set.
 *
 * Equivalent to `self.count() == 0`, but this function is faster.
 *
 * Examples:
 * ```
 * use shogi_core::{Bitboard, Square};
 * let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
 * let sq55 = Bitboard::single(Square::new(5, 5).unwrap());
 * assert!(!(sq11 | sq55).is_empty());
 * assert!(Bitboard::empty().is_empty());
 * ```
 */
bool Bitboard_is_empty(struct Bitboard self);

/**
 * C interface of `Bitboard::not`.
 */
struct Bitboard Bitboard_not(struct Bitboard a);

/**
 * C interface of [`Bitboard::pop`].
 */
OptionSquare Bitboard_pop(struct Bitboard *self);

/**
 * Creates a [`Bitboard`] with a single element.
 *
 * Examples:
 * ```
 * use shogi_core::{Bitboard, Square};
 * let sq11 = Bitboard::single(Square::new(1, 1).unwrap());
 * assert_eq!(sq11.count(), 1);
 * ```
 */
struct Bitboard Bitboard_single(Square square);

/**
 * Flips the player.
 *
 * Examples:
 * ```
 * use shogi_core::Color;
 * assert_eq!(Color::Black.flip(), Color::White);
 * assert_eq!(Color::White.flip(), Color::Black);
 * ```
 */
Color Color_flip(Color self);

/**
 * Creates a drop move.
 *
 * Examples:
 * ```
 * # use shogi_core::{Color, CompactMove, Move, Piece, PieceKind, Square};
 * let piece = Piece::new(PieceKind::Gold, Color::White);
 * let to = Square::new(3, 4).unwrap();
 * assert_eq!(<CompactMove as From<Move>>::from(Move::Drop { piece, to }), CompactMove::drop(piece, to));
 * ```
 */
CompactMove CompactMove_drop(Piece piece,
                             Square to);

/**
 * C interface of [`CompactMove::from`].
 */
OptionSquare CompactMove_from(CompactMove self);

/**
 * Finds whether `self` is a drop move.
 */
bool CompactMove_is_drop(CompactMove self);

/**
 * Finds whether `self` promotes a piece.
 */
bool CompactMove_is_promoting(CompactMove self);

/**
 * Creates a normal move.
 *
 * Examples:
 * ```
 * # use shogi_core::{CompactMove, Move, Square};
 * let from = Square::new(1, 2).unwrap();
 * let to = Square::new(3, 4).unwrap();
 * let promote = false;
 * assert_eq!(<CompactMove as From<Move>>::from(Move::Normal { from, to, promote }), CompactMove::normal(from, to, promote));
 * ```
 */
CompactMove CompactMove_normal(Square from,
                               Square to,
                               bool promote);

/**
 * Finds the `to` square.
 */
Square CompactMove_to(CompactMove self);

/**
 * C interface of [`Hand::added`].
 *
 * This function returns true if and only if adding was successful.
 */
bool Hand_add(struct Hand *self, PieceKind piece_kind);

/**
 * C interface of [`Hand::count`].
 *
 * This function returns true if and only if `piece_kind` can be a piece in hand.
 */
uint8_t Hand_count(struct Hand self, PieceKind piece_kind);

/**
 * Creates an empty instance of [`Hand`].
 *
 * Examples:
 * ```
 * use shogi_core::Hand;
 * assert_eq!(Hand::new(), Hand::default());
 * ```
 */
struct Hand Hand_new(void);

/**
 * C interface of [`Hand::removed`].
 *
 * This function returns true if and only if removal was successful.
 */
bool Hand_remove(struct Hand *self, PieceKind piece_kind);

struct Hand PartialPosition_hand_of_a_player(const struct PartialPosition *self, Color color);

OptionCompactMove PartialPosition_last_compact_move(const struct PartialPosition *self);

/**
 * Makes a move. This function is a C-compatible counterpart of `make_move`.
 * Note that this function will never check legality.
 *
 * Returns true if the given move makes sense, i.e.,
 * moves a piece to another square or drops a piece on a vacant square.
 *
 * If it returns false, it is guaranteed that self is not modified.
 */
bool PartialPosition_make_compact_move(struct PartialPosition *self, CompactMove mv);

OptionPiece PartialPosition_piece_at(const struct PartialPosition *self, Square square);

/**
 * Finds the subset of squares where a piece is placed.
 */
struct Bitboard PartialPosition_piece_bitboard(const struct PartialPosition *self, Piece piece);

/**
 * Finds the subset of squares where a piece of the specified player is placed.
 */
struct Bitboard PartialPosition_player_bitboard(const struct PartialPosition *self, Color color);

/**
 * Finds how many moves were made.
 */
uint16_t PartialPosition_ply(const struct PartialPosition *self);

/**
 * Finds which player is to move.
 */
Color PartialPosition_side_to_move(const struct PartialPosition *self);

/**
 * C interface of `startpos`.
 */
void PartialPosition_startpos(struct PartialPosition *buf);

/**
 * C interface of `to_sfen`.
 *
 * # Safety
 * This function writes to `ptr` at most 139 (= 129 + 1 + 1 + 1 + 0 + 1 + 5 + 1) bytes.
 * Caller should ensure that `ptr` has enough space for that.
 */
void PartialPosition_to_sfen_c(const struct PartialPosition *self, uint8_t *ptr);

/**
 * Finds the subset of squares with no pieces.
 */
struct Bitboard PartialPosition_vacant_bitboard(const struct PartialPosition *self);

/**
 * C interface of [`PieceKind::from_u8`].
 */
OptionPieceKind PieceKind_from_u8(uint8_t repr);

/**
 * Converts a [`u8`] to [`PieceKind`] without checking.
 *
 * # Safety
 * `repr` must be a valid representation of [`PieceKind`].
 * This condition is equivalent to `1 <= repr && repr <= 14`.
 */
PieceKind PieceKind_from_u8_unchecked(uint8_t repr);

/**
 * C interface of [`PieceKind::promote`].
 */
OptionPieceKind PieceKind_promote(PieceKind self);

/**
 * C interface of [`PieceKind::unpromote`].
 */
OptionPieceKind PieceKind_unpromote(PieceKind self);

/**
 * Finds the `Color` of this piece.
 */
Color Piece_color(Piece self);

/**
 * Creates a new `Piece` from `PieceKind` and `Color`.
 */
Piece Piece_new(PieceKind piece_kind, Color color);

/**
 * Finds the `PieceKind` of this piece.
 */
PieceKind Piece_piece_kind(Piece self);

/**
 * Promote a `Piece`. Same as `PieceKind::promote` with color.
 */
struct Option_Piece Piece_promote(Piece self);

/**
 * Un-promote a `Piece`. Same as `PieceKind::unpromote` with color.
 */
struct Option_Piece Piece_unpromote(Piece self);

/**
 * Destructs a `Position`.
 *
 * # Safety
 * `ptr` must be the one created by a function in this type.
 */
void Position_destruct(struct Position *ptr);

struct Hand Position_hand_of_a_player(const struct Position *self, Color color);

/**
 * Returns the initial position of [Position], i.e., the position before any moves given to it.
 */
const struct PartialPosition *Position_initial_position(const struct Position *self);

/**
 * Returns the inner `PartialPosition`.
 */
const struct PartialPosition *Position_inner(const struct Position *self);

OptionCompactMove Position_last_compact_move(const struct Position *self);

/**
 * Makes a move. This function is a C-compatible counterpart of `make_move`.
 * Note that this function will never check legality.
 *
 * Returns true if the given move makes sense, i.e.,
 * moves a piece to another square or drops a piece on a vacant square.
 *
 * If it returns false, it is guaranteed that self is not modified.
 */
bool Position_make_compact_move(struct Position *self, CompactMove mv);

OptionPiece Position_piece_at(const struct Position *self, Square square);

/**
 * Finds the subset of squares where a piece is placed.
 */
struct Bitboard Position_piece_bitboard(const struct Position *self, Piece piece);

/**
 * Finds the subset of squares where a piece of the specified player is placed.
 */
struct Bitboard Position_player_bitboard(const struct Position *self, Color color);

/**
 * Finds how many moves were made.
 */
uint16_t Position_ply(const struct Position *self);

/**
 * Finds which player is to move.
 */
Color Position_side_to_move(const struct Position *self);

/**
 * C interface of `startpos`.
 */
struct Position *Position_startpos(void);

/**
 * C interface of `to_sfen`.
 *
 * # Safety
 * This function writes to `ptr` at most 139 (= 129 + 1 + 1 + 1 + 0 + 1 + 5 + 1) bytes.
 * Caller should ensure that `ptr` has enough space for that.
 */
void Position_to_sfen_c(const struct Position *self, uint8_t *ptr);

/**
 * Finds the subset of squares with no pieces.
 */
struct Bitboard Position_vacant_bitboard(const struct Position *self);

/**
 * Finds the file in range `1..=9`.
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::new(3, 4).unwrap().file(), 3);
 * ```
 */
uint8_t Square_file(Square self);

/**
 * Finds the reflected square of `self`.
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::new(1, 1).unwrap().flip(), Square::new(9, 9).unwrap());
 * assert_eq!(Square::new(3, 4).unwrap().flip(), Square::new(7, 6).unwrap());
 * ```
 */
Square Square_flip(Square self);

/**
 * Converts a [`u8`] to a [`Square`]. If `value` is not in range `1..=81`, this function returns [`None`].
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::from_u8(21), Square::new(3, 3));
 * assert_eq!(Square::from_u8(0), None);
 * assert_eq!(Square::from_u8(82), None);
 * ```
 */
struct Option_Square Square_from_u8(uint8_t value);

/**
 * Converts [`u8`] to [`Square`] without checking.
 *
 * # Safety
 * `value` must be in range 1..=81
 */
Square Square_from_u8_unchecked(uint8_t value);

/**
 * Finds the index of `self` in range `1..=81`.
 * It is guaranteed that the result is equal to the internal representation, `9 * file + rank - 9`.
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::new(3, 4).unwrap().index(), 22);
 * ```
 */
uint8_t Square_index(Square self);

/**
 * Creates a new [`Square`] with given `file` and `rank`.
 *
 * `file` and `rank` must be between 1 and 9 (both inclusive).
 * If this condition is not met, this function returns None.
 */
struct Option_Square Square_new(uint8_t file, uint8_t rank);

/**
 * Creates a new [`Square`] with given `file`, `rank` and `color`.
 *
 * `file` and `rank` must be between 1 and 9 (both inclusive).
 * If this condition is not met, this function returns None.
 *
 * Examples:
 * ```
 * use shogi_core::{Color, Square};
 * assert_eq!(Square::new_relative(3, 4, Color::Black), Square::new(3, 4));
 * assert_eq!(Square::new_relative(3, 4, Color::White), Square::new(7, 6));
 * ```
 */
struct Option_Square Square_new_relative(uint8_t file, uint8_t rank, Color color);

/**
 * Finds the rank in range `1..=9`.
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::new(3, 4).unwrap().rank(), 4);
 * ```
 */
uint8_t Square_rank(Square self);

/**
 * Finds the file from the perspective of `color`.
 */
uint8_t Square_relative_file(Square self, Color color);

/**
 * Finds the rank from the perspective of `color`.
 */
uint8_t Square_relative_rank(Square self, Color color);

/**
 * Shifts `self` by the given arguments. If the result would be out of the board, this function returns [`None`].
 *
 * Examples:
 * ```
 * use shogi_core::Square;
 * assert_eq!(Square::new(3, 3).unwrap().shift(-1, 3), Square::new(2, 6));
 * assert_eq!(Square::new(8, 4).unwrap().shift(0, -3), Square::new(8, 1));
 * assert_eq!(Square::new(3, 3).unwrap().shift(-4, 3), None);
 * ```
 */
struct Option_Square Square_shift(Square self,
                                  int8_t file_delta,
                                  int8_t rank_delta);

#endif /* shogi_core_bindings_h */
