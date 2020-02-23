//! OS-specific extensions.

// TODO: Make these attributes working
//#[cfg(any(unix, doc))]
//#[cfg_attr(docsrs, doc(cfg(unix)))]
#[cfg(unix)]
pub mod unix;

// TODO: These too
//#[cfg(any(target_os = "macos", doc))]
//#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
#[cfg(target_os = "macos")]
pub mod macos;

// TODO: Add `cfg(doc)` and make in render in any non-Windows OS
#[cfg(target_os = "windows")]
#[cfg_attr(docsrs, doc(cfg(target_os = "windows")))]
pub mod windows;
