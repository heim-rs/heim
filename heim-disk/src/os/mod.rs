//! OS-specific extension for crate types.

#[cfg(any(doc, unix))]
pub mod unix;

cfg_if::cfg_if! {
    if #[cfg(any(doc, target_os = "macos"))] {
        pub mod macos;
    }
}
