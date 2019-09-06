//! OS-specific extensions.
//!
//! These are not cross-platform and their usage should be `cfg`-wrapped.

#[cfg(unix)]
pub mod unix;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
    }
}
