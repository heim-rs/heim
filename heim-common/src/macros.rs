//! Inner macros used across `heim-*` sub-crates.

/// Generate boilerplate code for newtypes hiding the platform-specific implementations.
#[macro_export]
macro_rules! wrap {
    ($type:path, $inner:path) => {
        #[doc(hidden)]
        impl AsRef<$inner> for $type {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        #[doc(hidden)]
        impl From<$inner> for $type {
            fn from(inner: $inner) -> $type {
                $type(inner)
            }
        }
    };
    ($lifetime:tt, $type:path, $inner:path) => {
        #[doc(hidden)]
        impl<$lifetime> AsRef<$inner> for $type {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        #[doc(hidden)]
        impl<$lifetime> From<$inner> for $type {
            fn from(inner: $inner) -> $type {
                $type(inner)
            }
        }
    };
}
