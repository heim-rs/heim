cfg_if::cfg_if! {
    if #[cfg(unix)] {
        // Since there is a lot of shared into between all UNIX-like systems,
        // aggregating them into a separate module.
        mod unix;

        pub use self::unix::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub use self::macos::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub use self::windows::*;
    } else {
        compile_error!("Unsupported OS");
    }
}
