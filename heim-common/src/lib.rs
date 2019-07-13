//! This crate shares common functionality across the `heim` sub-crates.
//!
//! Do **NOT** use it directly.

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

mod errors;
pub mod sys;
pub mod units;
pub mod utils;

pub use self::errors::{Error, Result};

/// Prelude intended to be used across `heim-*` crates.
///
/// Consider not to use it in your code, because it is kinda internal
/// and might change at any time.
pub mod prelude {
    pub use super::errors::{Error, Result};
    pub use super::utils;
    pub use futures::future::{FutureExt, TryFutureExt};
    pub use futures::prelude::*;
    pub use futures::stream::{StreamExt, TryStreamExt};
}
