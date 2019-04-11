#[macro_use]
extern crate uom;

mod errors;
pub mod units;
pub mod utils;

pub use self::errors::{Error, ErrorKind, Result};

/// Prelude intended to be used across `heim-*` crates.
///
/// Consider not to use it in your code, because it is kinda internal
/// and might change at any time.
pub mod prelude {
    pub use super::errors::{Error, ErrorKind, Result};

    pub use tokio::prelude::*;

    pub use super::utils;
}
