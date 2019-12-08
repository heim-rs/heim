//! This crate shares common functionality across the `heim` sub-crates.
//!
//! Do **NOT** use it directly.

#![doc(html_root_url = "https://docs.rs/heim-common/0.0.9")]
#![deny(
    unused,
    unused_imports,
    unused_features,
    bare_trait_objects,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]
#![allow(clippy::missing_safety_doc)]

mod errors;
#[doc(hidden)]
pub mod sys;
pub mod units;
#[doc(hidden)]
pub mod utils;

pub use self::errors::{Error, Result};

/// Process identifier type.
#[cfg(unix)]
pub type Pid = libc::pid_t;

/// Process identifier type.
// TODO: Is it a correct type for pid?
#[cfg(target_os = "windows")]
pub type Pid = winapi::shared::minwindef::DWORD;

/// Prelude intended to be used across `heim-*` crates.
///
/// Consider not to use it in your code, because it is kinda internal
/// and might change at any time.
pub mod prelude {
    pub use super::errors::{Error, Result};

    /// This module tries to mimic `futures` crate structure
    /// except without re-exporting unused subcrates like `executor` or `compat`.
    pub mod futures {
        pub use futures_util::ready;
        pub use futures_util::task;

        /// Asynchronous values.
        pub mod future {
            pub use futures_core::future::*;
            pub use futures_util::future::*;
        }

        /// Asynchronous streams.
        pub mod stream {
            pub use futures_core::stream::*;
            pub use futures_util::stream::*;
        }
    }

    // And these re-exports are used across the `heim-*` crates,
    // would be bad to break them
    pub use self::futures::future::{
        self, BoxFuture, FusedFuture, Future, FutureExt, TryFutureExt,
    };
    pub use self::futures::stream::{
        self, BoxStream, FusedStream, Stream, StreamExt, TryStreamExt,
    };
}
