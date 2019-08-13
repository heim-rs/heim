//! OS-specific routines for `heim` sub-crates (and `heim` sub-crates only).
//!
//! Do not use them.

#[cfg(unix)]
pub mod unix;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub mod windows;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
    }
}
