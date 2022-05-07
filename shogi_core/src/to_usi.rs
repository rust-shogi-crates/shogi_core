use core::fmt::{Result as FmtResult, Write};

/// A type that is convertible to USI format.
pub trait ToUsi {
    /// Write `self` in USI format.
    fn to_usi<W: Write>(&self, sink: &mut W) -> FmtResult;

    /// Returns `self`'s string representation.
    #[cfg(feature = "alloc")]
    fn to_usi_owned(&self) -> alloc::string::String {
        let mut s = alloc::string::String::new();
        self.to_usi(&mut s).unwrap();
        s
    }
}
