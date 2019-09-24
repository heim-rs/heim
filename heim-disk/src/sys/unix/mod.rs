#[cfg(not(target_os = "linux"))] // Linux have it's own way
mod partitions;
mod usage;

#[cfg(not(target_os = "linux"))]
mod bindings;

#[cfg(not(target_os = "linux"))]
pub use self::partitions::*;
pub use self::usage::*;
