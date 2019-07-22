//! OS-specific extensions for public types.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
    }
}
