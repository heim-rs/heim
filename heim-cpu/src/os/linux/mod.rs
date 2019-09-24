//! Linux-specific extensions.
//!
//! Available only for `cfg(target_os = "linux")`

mod freq;
mod stats;
mod times;

pub use self::freq::*;
pub use self::stats::*;
pub use self::times::*;
