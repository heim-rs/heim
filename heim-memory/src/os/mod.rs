//! OS-specific extensions.
//!
//! These are not cross-platform and their usage should be `cfg`-wrapped.

#[cfg(not(windows))]
use heim_common::units::Information;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
    }
}

/// OS-specific extension to [Swap].
///
/// ## Compatibility
///
/// Applicable for all supported platforms except Windows.
///
/// [Swap]: ../struct.Swap.html
#[cfg(not(windows))]
pub trait SwapExt {
    /// The cumulative amount of information the system has swapped in from disk.
    fn sin(&self) -> Option<Information>;

    /// The cumulative amount of information the system has swapped out from disk.
    fn sout(&self) -> Option<Information>;
}

#[cfg(not(windows))]
impl SwapExt for crate::Swap {
    fn sin(&self) -> Option<Information> {
        self.as_ref().sin()
    }

    fn sout(&self) -> Option<Information> {
        self.as_ref().sout()
    }
}
