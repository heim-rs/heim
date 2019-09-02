//! OS-specific routines for `heim` sub-crates (and `heim` sub-crates only).
//!
//! Do not use them.

use crate::units::Time;

/// Converting various OS structs into the `Time` unit.
pub trait IntoTime {
    /// Do the magic
    fn into_time(self) -> Time;
}

#[cfg(unix)]
pub mod unix;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub mod windows;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
    }
}
