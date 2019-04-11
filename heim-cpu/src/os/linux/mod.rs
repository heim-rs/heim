//! Linux-specific extensions.
//!
//! Available only for `cfg(target_os = "linux")`
mod times;
mod freq;

pub use self::times::*;
pub use self::freq::*;
