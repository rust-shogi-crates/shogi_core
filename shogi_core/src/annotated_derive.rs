// Current nightly rustc (cd282d7f7 2022-05-18) does not support annotations for `derive`-based implementations.
// Therefore, we need to impl these traits manually in order to feature-gate their implementations.
// This module helps these implementations by providing boilerplate impls.
// Ref: https://github.com/rust-lang/rust/issues/43781
// TODO: Use `derive` when annotations for derived impls become available

// Defines a Hash implementation for a tuple struct with a single field.
macro_rules! impl_hash_for_single_field {
    ($ty:ty) => {
        #[cfg(feature = "hash")]
        #[cfg_attr(docsrs, doc(cfg(feature = "hash")))]
        impl core::hash::Hash for $ty {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}

macro_rules! impl_hash_with_fields {
    ($ty:ty; $($field:ident),+) => {
        #[cfg(feature = "hash")]
        #[cfg_attr(docsrs, doc(cfg(feature = "hash")))]
        impl core::hash::Hash for $ty {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                $(self.$field.hash(state);)+
            }
        }
    };
}

// Defines a Hash implementation for a fieldless enum.
macro_rules! impl_hash_for_fieldless_enum {
    ($ty:ty) => {
        #[cfg(feature = "hash")]
        #[cfg_attr(docsrs, doc(cfg(feature = "hash")))]
        impl core::hash::Hash for $ty {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
    };
}

// impl PartialOrd and Ord for a tuple struct with a single field
macro_rules! impl_ord_for_single_field {
    ($ty:ty) => {
        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::Ord for $ty {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    };
}

macro_rules! impl_ord_with_fields {
    ($ty:ty; $($field:ident),+) => {
        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::Ord for $ty {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                ($(&self.$field),+).cmp(&($(&other.$field),+))
            }
        }
    };
}

// impl PartialOrd and Ord for a fieldless enum
macro_rules! impl_ord_for_fieldless_enum {
    ($ty:ty) => {
        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                (*self as u8).partial_cmp(&(*other as u8))
            }
        }

        #[cfg(feature = "ord")]
        #[cfg_attr(docsrs, doc(cfg(feature = "ord")))]
        impl core::cmp::Ord for $ty {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                (*self as u8).cmp(&(*other as u8))
            }
        }
    };
}
