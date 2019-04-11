//! OS-specific extension for crate types.

cfg_if::cfg_if! {
    if #[cfg(any(doc, unix))] {
        pub mod unix;
    }
}
