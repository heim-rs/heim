//! macOS-specific extensions.
//!
//! Available only for `cfg(target_os = "macos")`

mod stats;

pub use self::stats::*;
