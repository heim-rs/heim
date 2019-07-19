//! macOS-specific extensions.
//!
//! Available only for `cfg(target_os = "windows")`

mod stats;

pub use self::stats::*;
