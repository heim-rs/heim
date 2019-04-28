mod usage;
#[cfg(not(target_os = "linux"))]  // Linux have it's own way
mod partitions;

#[cfg(not(target_os = "linux"))]
mod bindings;

pub use self::usage::*;
#[cfg(not(target_os = "linux"))]
pub use self::partitions::*;
