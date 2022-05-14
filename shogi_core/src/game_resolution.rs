/// How a game is resolved.
///
/// [`GameResolution`] and <code>[Option]<[GameResolution]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub enum GameResolution {
    /// White's king was mated or white resigned.
    ///
    /// Discriminant = 1.
    BlackWins = 1,
    /// Black's king was mated or black resigned.
    ///
    /// Discriminant = 2.
    WhiteWins = 2,
    /// This can happen if e.g. `持将棋` (*jishōgi*) happens.
    ///
    /// Discriminant = 3.
    Draw = 3,
    /// This can happen if e.g. `千日手` (*sennichite*, repetition) happens.
    ///
    /// Discriminant = 4.
    Rematch = 4,
    /// The game was aborted.
    ///
    /// Discriminant = 5.
    Aborted = 5,
}

impl GameResolution {
    /// Converts a [`u8`] to [`GameResolution`] without checking.
    ///
    /// # Safety
    /// `repr` must be a valid representation of [`GameResolution`].
    /// This condition is equivalent to `1 <= repr && repr <= 5`.
    #[export_name = "GameResolution_from_u8_unchecked"]
    pub unsafe extern "C" fn from_u8_unchecked(repr: u8) -> Self {
        core::mem::transmute(repr)
    }
}

/// <code>[Option]<[GameResolution]></code> with defined representation.
///
/// The representation is:
/// [`None`] => `0`, <code>[Some]\(x\)</code> => `x`.
/// Therefore, valid representations of this type are precisely `0..=5`.
///
/// This type is provided for C interoperability.
/// cbindgen cannot deduce that <code>[Option]<[GameResolution]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
/// Users of this type should convert to/from <code>[Option]<[GameResolution]></code>.
///
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct OptionGameResolution(u8);

impl From<Option<GameResolution>> for OptionGameResolution {
    #[inline(always)]
    fn from(arg: Option<GameResolution>) -> Self {
        Self(match arg {
            Some(result) => result as u8,
            None => 0,
        })
    }
}

impl From<OptionGameResolution> for Option<GameResolution> {
    #[inline(always)]
    fn from(arg: OptionGameResolution) -> Self {
        if arg.0 == 0 {
            None
        } else {
            // Safety: arg is a valid OptionGameResolution, which means 0 <= arg.0 && arg.0 <= 5.
            // arg.0 == 0 is ruled out.
            Some(unsafe { GameResolution::from_u8_unchecked(arg.0) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_resolution_is_one_byte() {
        assert_eq!(core::mem::size_of::<GameResolution>(), 1);
    }

    #[test]
    fn option_game_resolution_default_is_compatible() {
        // Option<T>'s default value is [`None`].
        assert_eq!(OptionGameResolution::default(), None.into());
    }
}
