#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    }
}
