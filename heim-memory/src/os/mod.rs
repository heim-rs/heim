//! OS-specific extensions.

#[cfg(any(not(target_os = "windows"), doc))]
use heim_common::units::Information;

#[cfg(any(target_os = "linux", doc))]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
pub mod linux;

#[cfg(any(target_os = "macos", doc))]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub mod macos;

/// OS-specific extension to [Swap].
///
/// ## Compatibility
///
/// Applicable for all supported platforms except Windows.
///
/// [Swap]: ../struct.Swap.html
#[cfg(any(not(target_os = "windows"), doc))]
#[cfg_attr(docsrs, doc(cfg(not(target_os = "windows"))))]
pub trait SwapExt {
    /// The cumulative amount of information the system has swapped in from disk.
    fn sin(&self) -> Option<Information>;

    /// The cumulative amount of information the system has swapped out from disk.
    fn sout(&self) -> Option<Information>;
}

#[cfg(not(target_os = "windows"))]
impl SwapExt for crate::Swap {
    fn sin(&self) -> Option<Information> {
        self.as_ref().sin()
    }

    fn sout(&self) -> Option<Information> {
        self.as_ref().sout()
    }
}
