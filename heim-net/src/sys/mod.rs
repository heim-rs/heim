#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub use self::macos::*;
    }
}
