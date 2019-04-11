#![allow(missing_docs)]

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;

    } else if #[cfg(windows)] {
        mod windows;

        pub use self::windows::*;
    }
}
