//! OS-specific extensions.

#[cfg(any(target_os = "linux", doc))]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
pub mod linux;

#[cfg(any(target_os = "macos", doc))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub mod macos;

#[cfg(any(target_os = "windows", doc))]
#[cfg_attr(docsrs, doc(cfg(target_os = "windows")))]
pub mod windows;
