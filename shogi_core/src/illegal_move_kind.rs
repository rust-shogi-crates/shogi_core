/// Kinds of illegal moves.
///
/// [`IllegalMoveKind`] and <code>[Result]<[()][unit], [IllegalMoveKind]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
///
/// Note: the equality of sizes are not guaranteed, but assumed to be correct.
#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum IllegalMoveKind {
    /// A player has two pawns in the same file. Promoted pawns are not counted.
    ///
    /// Discriminant = 1.
    TwoPawns = 1,
    /// A player ignored a check.
    ///
    /// Discriminant = 2.
    IgnoredCheck = 2,
    /// A drop-pawn-mate (`打ち歩詰め`, *uchifu-zume*).
    ///
    /// Discriminant = 3.
    DropPawnMate = 3,
    /// A drop move is stuck.
    ///
    /// Discriminant = 4.
    DropStuck = 4,
    /// A normal move is stuck.
    ///
    /// Discriminant = 5.
    NormalStuck = 5,
    /// A player made a move even after the game finished.
    ///
    /// Discriminant = 6.
    GameFinished = 6,
    /// Incorrect move.
    ///
    /// Discriminant = 7.
    IncorrectMove = 7,
}

impl IllegalMoveKind {
    /// Converts a [`u8`] to [`IllegalMoveKind`] if possible.
    ///
    /// If `repr` is a valid representation of [`IllegalMoveKind`], this function returns `Some(illegal_move_kind)`.
    /// This condition is equivalent to `1 <= repr && repr <= 7`.
    pub fn from_u8(repr: u8) -> Option<Self> {
        if matches!(repr, 1..=7) {
            // Safety: `repr` is in range `1..=7`.
            Some(unsafe { Self::from_u8_unchecked(repr) })
        } else {
            None
        }
    }

    /// Converts a [`u8`] to [`IllegalMoveKind`] without checking.
    ///
    /// # Safety
    /// `repr` must be a valid representation of [`IllegalMoveKind`].
    /// This condition is equivalent to `1 <= repr && repr <= 7`.
    #[export_name = "IllegalMoveKind_from_u8_unchecked"]
    pub unsafe extern "C" fn from_u8_unchecked(repr: u8) -> Self {
        core::mem::transmute(repr)
    }
}

impl_ord_for_fieldless_enum!(IllegalMoveKind);
impl_hash_for_fieldless_enum!(IllegalMoveKind);

/// <code>[Result]<[()][unit], [IllegalMoveKind]></code> with defined representation.
///
/// The representation is:
/// <code>[`Ok`](())</code> => `0`, <code>[Err]\(x\)</code> => `x`.
/// Therefore, valid representations of this type are precisely `0..=7`.
///
/// This type is provided for C interoperability.
/// cbindgen cannot deduce that <code>[Result]<[()][unit], [IllegalMoveKind]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
/// Users of this type should convert to/from <code>[Result]<[()][unit], [IllegalMoveKind]></code>.
///
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct ResultUnitIllegalMoveKind(u8);

impl From<Option<IllegalMoveKind>> for ResultUnitIllegalMoveKind {
    #[inline(always)]
    fn from(arg: Option<IllegalMoveKind>) -> Self {
        Self(match arg {
            Some(result) => result as u8,
            None => 0,
        })
    }
}

impl From<ResultUnitIllegalMoveKind> for Option<IllegalMoveKind> {
    #[inline(always)]
    fn from(arg: ResultUnitIllegalMoveKind) -> Self {
        if arg.0 == 0 {
            None
        } else {
            // Safety: arg is a valid OptionIllegalMoveKind, which means 0 <= arg.0 && arg.0 <= 7.
            // arg.0 == 0 is ruled out.
            Some(unsafe { IllegalMoveKind::from_u8_unchecked(arg.0) })
        }
    }
}

impl From<Result<(), IllegalMoveKind>> for ResultUnitIllegalMoveKind {
    #[inline(always)]
    fn from(arg: Result<(), IllegalMoveKind>) -> Self {
        Self(match arg {
            Err(result) => result as u8,
            Ok(()) => 0,
        })
    }
}

impl From<ResultUnitIllegalMoveKind> for Result<(), IllegalMoveKind> {
    #[inline(always)]
    fn from(arg: ResultUnitIllegalMoveKind) -> Self {
        if arg.0 == 0 {
            Ok(())
        } else {
            // Safety: arg is a valid OptionIllegalMoveKind, which means 0 <= arg.0 && arg.0 <= 7.
            // arg.0 == 0 is ruled out.
            Err(unsafe { IllegalMoveKind::from_u8_unchecked(arg.0) })
        }
    }
}

impl_ord_for_single_field!(ResultUnitIllegalMoveKind);
impl_hash_for_single_field!(ResultUnitIllegalMoveKind);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn illegal_move_kind_is_one_byte() {
        assert_eq!(core::mem::size_of::<IllegalMoveKind>(), 1);
    }

    #[test]
    fn option_illegal_move_kind_default_is_compatible() {
        // Option<T>'s default value is [`None`].
        assert_eq!(ResultUnitIllegalMoveKind::default(), None.into());
    }

    #[test]
    fn repr_is_correct() {
        for repr in 1..=7 {
            let value = IllegalMoveKind::from_u8(repr).unwrap();
            assert_eq!(value as u8, repr);
        }
    }

    #[test]
    fn from_works() {
        for repr in 1..=7 {
            let value = IllegalMoveKind::from_u8(repr).unwrap();
            assert_eq!(
                Result::from(ResultUnitIllegalMoveKind::from(Err(value))),
                Err(value),
            );
            assert_eq!(ResultUnitIllegalMoveKind::from(Err(value)).0, repr);
        }
    }
}
