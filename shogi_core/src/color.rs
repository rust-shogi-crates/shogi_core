use crate::ToUsi;

/// A player.
///
/// [`Color`] and <code>[Option]<[Color]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Color {
    /// Black, who plays first. Known as `先手` (*sente*).
    ///
    /// Its representation is 1.
    Black = 1,
    /// White, who plays second. Known as `後手` (*gote*).
    ///
    /// Its representation is 2.
    White = 2,
}

impl Color {
    /// Flips the player.
    ///
    /// Examples:
    /// ```
    /// use shogi_core::Color;
    /// assert_eq!(Color::Black.flip(), Color::White);
    /// assert_eq!(Color::White.flip(), Color::Black);
    /// ```
    #[export_name = "Color_flip"]
    pub extern "C" fn flip(self) -> Self {
        // The shortest possible machine code for this function in x86_64 (System V AMD64 ABI) is:
        // 89 f8  movl %edi, %eax
        // 34 03  xorb $3, %al
        unsafe { core::mem::transmute(self as u8 ^ 3) }
    }

    /// Returns the index of `self` for array accesses. This function returns an integer in range `0..Color::MAX`.
    ///
    /// Since: 0.1.2
    #[inline]
    pub const fn array_index(self) -> usize {
        self as usize - 1
    }

    /// How many elements should an array indexed by [`Color`] have?
    ///
    /// Examples:
    /// ```
    /// # use shogi_core::Color;
    /// // values is long enough so values[color.index()] never panics
    /// let mut values = [0; Color::NUM];
    /// values[Color::Black.array_index()] = 10;
    /// values[Color::White.array_index()] = -10;
    /// ```
    /// Since: 0.1.2
    pub const NUM: usize = 2;

    /// Returns all possible `Color`s in the ascending order of their discriminants.
    pub fn all() -> [Self; 2] {
        [Color::Black, Color::White]
    }
}

impl_ord_for_fieldless_enum!(Color);
impl_hash_for_fieldless_enum!(Color);

impl ToUsi for Color {
    fn to_usi<W: core::fmt::Write>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_str(match *self {
            Color::Black => "b",
            Color::White => "w",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_works() {
        let colors = Color::all();
        assert_eq!(colors[0].flip(), colors[1]);
        assert_eq!(colors[1].flip(), colors[0]);
    }

    #[test]
    fn array_index_works() {
        for i in 0..2 {
            assert_eq!(Color::all()[i].array_index(), i);
        }
    }
}
