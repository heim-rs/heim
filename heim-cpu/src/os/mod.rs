//! OS-specific extensions

cfg_if::cfg_if! {
    if #[cfg(any(doc, target_os = "linux"))] {
        pub mod linux;
    }
}
