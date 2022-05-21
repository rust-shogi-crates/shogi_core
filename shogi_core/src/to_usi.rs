use core::fmt::{Result as FmtResult, Write};

/// A type that is convertible to USI format.
pub trait ToUsi {
    /// Write `self` in USI format.
    ///
    /// This function returns Err(core::fmt::Error)
    /// if and only if it fails to write to `sink`.
    fn to_usi<W: Write>(&self, sink: &mut W) -> FmtResult;

    /// Returns `self`'s string representation.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn to_usi_owned(&self) -> alloc::string::String {
        let mut s = alloc::string::String::new();
        // guaranteed to be Ok(())
        let result = self.to_usi(&mut s);
        debug_assert_eq!(result, Ok(()));
        s
    }
}
