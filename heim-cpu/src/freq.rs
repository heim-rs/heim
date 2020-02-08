use std::fmt;

use heim_common::prelude::*;
use heim_common::units::Frequency;

use crate::sys;

/// System CPU frequency.
pub struct CpuFrequency(sys::CpuFrequency);

wrap!(CpuFrequency, sys::CpuFrequency);

impl CpuFrequency {
    /// Current CPU frequency.
    ///
    /// ## Compatibility
    ///
    /// On Linux it returns the real-time value, on all other platforms
    /// it represents the nominal "fixed" value.
    pub fn current(&self) -> Frequency {
        self.as_ref().current()
    }

    /// Minimal CPU frequency.
    ///
    /// ## Returns
    ///
    /// Returns `None` if value can't be determined.
    pub fn min(&self) -> Option<Frequency> {
        self.as_ref().min()
    }

    /// Maximal CPU frequency.
    ///
    /// ## Returns
    ///
    /// Returns `None` if value can't be determined.
    pub fn max(&self) -> Option<Frequency> {
        self.as_ref().max()
    }
}

impl fmt::Debug for CpuFrequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuFrequency")
            .field("current", &self.current())
            .field("min", &self.min())
            .field("max", &self.max())
            .finish()
    }
}

/// Returns future which will resolve into [CpuFrequency].
///
/// ## Compatibility
///
/// Per-CPU frequencies retrieval is available for Linux,
/// see Linux-specific [frequencies] function.
///
/// [CpuFrequency]: ./struct.CpuFrequency.html
/// [frequencies]: ./os/linux/fn.frequencies.html
pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    sys::frequency().map_ok(Into::into)
}
