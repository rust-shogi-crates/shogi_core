/// Kinds of pieces.
///
/// [`PieceKind`] and <code>[Option]<[PieceKind]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PieceKind {
    /// A pawn. Unlike in chess, it always moves one square forward,
    /// even if the destination square is occuipied by an enemy piece.
    ///
    /// Known as `歩` (*fu*) or `歩兵` (*fuhyō*), although the latter name is rarely used.
    ///
    /// Discriminant = 1.
    Pawn = 1,
    /// A lance. It moves any number of squares forward without jumping over other pieces.
    /// Chess has no counterpart of it.
    ///
    /// Known as `香` (*kyō*) or `香車` (*kyōsha*).
    ///
    /// Discriminant = 2.
    Lance = 2,
    /// A knight. Unlike in chess, it can only move two squares forward and one square vertically.
    ///
    /// Known as `桂` (*kē*) or `桂馬` (*kēma*).
    ///
    /// Discriminant = 3.
    Knight = 3,
    /// A silver general. It moves one square forward or diagonally.
    /// Chess has no counterpart of it.
    ///
    /// Known as `銀` (*gin*) or `銀将` (*ginshō*), although the latter name is rarely used.
    ///
    /// Discriminant = 4.
    Silver = 4,
    /// A gold general. It moves one square horizontally, vertically, and diagonally forward.
    /// Chess has no counterpart of it.
    ///
    /// Known as `金` (*kin*) or `金将` (*kinshō*), although the latter name is rarely used.
    ///
    /// Discriminant = 5.
    Gold = 5,
    /// A bishop. It moves any number of squares diagonally,
    /// exactly the same way as a bishop does in chess.
    ///
    /// Known as `角` (*kaku*) or `角行` (*kakugyō*), although the latter name is rarely used.
    ///
    /// Discriminant = 6.
    Bishop = 6,
    /// A rook. It moves any number of squares horizontally or vertically.
    /// It is almost the same as a rook in chess, but shogi has no rule of castling.
    ///
    /// Known as `飛` (*hi*) or `飛車` (*hisha*), although the former name is rarely used to refer to a piece.
    ///
    /// Discriminant = 7.
    Rook = 7,
    /// A king. It moves one square horizontally, vertically or diagonally.
    /// A move that would expose the king to an enemy piece's capture threat is an illegal move,
    /// and the player that has no legal moves immediately loses.
    ///
    /// It is almost the same as a king in chess, but shogi has no rule of castling.
    ///
    /// Known as `王` (*ō*), `王将` (*ōshō*), `玉` (*gyoku*) or `玉将` (*gyokushō*).
    /// The two-letter names are rarely used to refer to pieces.
    ///
    /// Discriminant = 8.
    King = 8,
    /// A promoted pawn. Moves exactly the same way as a gold general.
    ///
    /// Known as `と` (*to*) or `と金` (*tokin*),
    /// although the former name is rarely used to refer to a piece.
    ///
    /// Discriminant = 9.
    ProPawn = 9,
    /// A promoted lance. Moves exactly the same way as a gold general.
    ///
    /// Known as `成香` (*narikyō*).
    ///
    /// Discriminant = 10.
    ProLance = 10,
    /// A promoted knight. Moves exactly the same way as a gold general.
    ///
    /// Known as `成桂` (*narikē*).
    ///
    /// Discriminant = 11.
    ProKnight = 11,
    /// A promoted silver general. Moves exactly the same way as a gold general.
    ///
    /// Known as `成銀` (*narigin*).
    ///
    /// Discriminant = 12.
    ProSilver = 12,
    /// A promoted bishop. It moves any number of squares diagonally, or one square horizontally or vertically.
    ///
    /// Known as `馬` (*uma*), `竜馬` (*ryūma*),
    /// although the latter is rarely used and confusing.
    ///
    /// Discriminant = 13.
    ProBishop = 13,
    /// A promoted rook.  It moves any number of squares horizontally or vertically, or one square diagonally.
    ///
    /// Known as `竜` (*ryū*), `竜王` (*ryūō*),
    /// although the latter is rarely used and confusing.
    ///
    /// Discriminant = 14.
    ProRook = 14,
}

impl PieceKind {
    /// Returns the promoted version of `self`.
    ///
    /// If `self` cannot promote, this function returns [`None`].
    #[must_use]
    #[inline]
    pub fn promote(self) -> Option<Self> {
        match self {
            PieceKind::Pawn => Some(PieceKind::ProPawn),
            PieceKind::Lance => Some(PieceKind::ProLance),
            PieceKind::Knight => Some(PieceKind::ProKnight),
            PieceKind::Silver => Some(PieceKind::ProSilver),
            PieceKind::Gold => None,
            PieceKind::Bishop => Some(PieceKind::ProBishop),
            PieceKind::Rook => Some(PieceKind::ProRook),
            PieceKind::King => None,
            PieceKind::ProPawn => None,
            PieceKind::ProLance => None,
            PieceKind::ProKnight => None,
            PieceKind::ProSilver => None,
            PieceKind::ProBishop => None,
            PieceKind::ProRook => None,
        }
    }
    /// Returns the un-promoted version of `self`. This function can also be used to check if a piece is promoted.
    ///
    /// If `self` is not a promoted piece, this function returns `None`.
    #[must_use]
    #[inline]
    pub fn unpromote(self) -> Option<Self> {
        match self {
            PieceKind::Pawn => None,
            PieceKind::Lance => None,
            PieceKind::Knight => None,
            PieceKind::Silver => None,
            PieceKind::Gold => None,
            PieceKind::Bishop => None,
            PieceKind::Rook => None,
            PieceKind::King => None,
            PieceKind::ProPawn => Some(PieceKind::Pawn),
            PieceKind::ProLance => Some(PieceKind::Lance),
            PieceKind::ProKnight => Some(PieceKind::Knight),
            PieceKind::ProSilver => Some(PieceKind::Silver),
            PieceKind::ProBishop => Some(PieceKind::Bishop),
            PieceKind::ProRook => Some(PieceKind::Rook),
        }
    }

    /// Converts a [`u8`] to [`PieceKind`] if possible.
    ///
    /// If `repr` is a valid representation of [`PieceKind`], this function returns `Some(piece_kind)`.
    /// This condition is equivalent to `1 <= repr && repr <= 14`.
    pub fn from_u8(repr: u8) -> Option<Self> {
        if matches!(repr, 1..=14) {
            // Safety: `repr` is in range `1..=14`.
            Some(unsafe { Self::from_u8_unchecked(repr) })
        } else {
            None
        }
    }

    /// C interface of [`PieceKind::from_u8`].
    #[no_mangle]
    pub extern "C" fn PieceKind_from_u8(repr: u8) -> OptionPieceKind {
        Self::from_u8(repr).into()
    }

    /// Converts a [`u8`] to [`PieceKind`] without checking.
    ///
    /// # Safety
    /// `repr` must be a valid representation of [`PieceKind`].
    /// This condition is equivalent to `1 <= repr && repr <= 14`.
    #[export_name = "PieceKind_from_u8_unchecked"]
    #[inline(always)]
    pub unsafe extern "C" fn from_u8_unchecked(repr: u8) -> Self {
        core::mem::transmute(repr)
    }

    /// C interface of [`PieceKind::promote`].
    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn PieceKind_promote(self) -> OptionPieceKind {
        self.promote().into()
    }

    /// C interface of [`PieceKind::unpromote`].
    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn PieceKind_unpromote(self) -> OptionPieceKind {
        self.unpromote().into()
    }

    /// Returns all possible `PieceKind`s in the ascending order of their discriminants.
    pub fn all() -> [Self; 14] {
        [
            PieceKind::Pawn,
            PieceKind::Lance,
            PieceKind::Knight,
            PieceKind::Silver,
            PieceKind::Gold,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::King,
            PieceKind::ProPawn,
            PieceKind::ProLance,
            PieceKind::ProKnight,
            PieceKind::ProSilver,
            PieceKind::ProBishop,
            PieceKind::ProRook,
        ]
    }
}

impl_ord_for_fieldless_enum!(PieceKind);
impl_hash_for_fieldless_enum!(PieceKind);

/// <code>[Option]<[PieceKind]></code> with defined representation.
///
/// The correspondence is:
/// [`None`] => `0`, <code>[Some]\(x\)</code> => `x`.
/// Therefore, valid representations of this type are precisely `0..=14`.
///
/// This type is provided for C interoperability.
/// cbindgen cannot deduce that <code>[Option]<[PieceKind]></code> can be represented by `uint16_t` in C, so we need to define the bridge type.
/// Users of this type should convert to/from <code>[Option]<[PieceKind]></code>.
///
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
pub struct OptionPieceKind(u8);

impl From<Option<PieceKind>> for OptionPieceKind {
    #[inline(always)]
    fn from(arg: Option<PieceKind>) -> Self {
        Self(match arg {
            Some(result) => result as u8,
            None => 0,
        })
    }
}

impl From<OptionPieceKind> for Option<PieceKind> {
    #[inline(always)]
    fn from(arg: OptionPieceKind) -> Self {
        if arg.0 == 0 {
            None
        } else {
            // Safety: arg is a valid OptionPieceKind, which means 0 <= arg.0 && arg.0 <= 14.
            // arg.0 == 0 is ruled out.
            Some(unsafe { PieceKind::from_u8_unchecked(arg.0) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_works() {
        for piece_kind in PieceKind::all() {
            assert_eq!(PieceKind::from_u8(piece_kind as u8), Some(piece_kind));
        }
    }
}
