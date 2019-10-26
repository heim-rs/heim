//! MacOS specific extensions for crate types.

use heim_common::units::Information;

/// MacOS-specific extension to [`Memory`]
pub trait MemoryExt {
    /// Returns memory currently in use or very recently used, and so it is in RAM.
    fn active(&self) -> Information;

    /// Returns memory that is marked as not used.
    fn inactive(&self) -> Information;

    /// Returns memory that is marked to always stay in RAM. It is never moved to disk.
    fn wire(&self) -> Information;
}

#[cfg(target_os = "macos")]
impl MemoryExt for crate::Memory {
    fn active(&self) -> Information {
        self.as_ref().active()
    }

    fn inactive(&self) -> Information {
        self.as_ref().inactive()
    }

    fn wire(&self) -> Information {
        self.as_ref().wire()
    }
}
