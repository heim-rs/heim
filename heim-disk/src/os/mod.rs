//! OS-specific extension for crate types.

#[cfg(unix)]
pub mod unix;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;
