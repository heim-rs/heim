//! This crate shares common functionality across the `heim` sub-crates.
//!
//! Do **NOT** use it directly.

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
