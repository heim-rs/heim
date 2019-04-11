cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    }
}
