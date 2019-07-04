use std::fmt;

use heim_common::prelude::*;

use crate::{sys, units};

/// System CPU frequency.
///
/// ## Compatibility
///
/// Per-CPU frequencies retrieval is available for Linux,
/// see [frequencies] function.
///
/// [frequencies]: crate::os::linux::frequencies;
#[derive(heim_derive::ImplWrap)]
pub struct CpuFrequency(sys::CpuFrequency);

impl CpuFrequency {
    /// Current CPU frequency.
    ///
    /// ## Compatibility
    ///
    /// On Linux it returns the real-time value, on all other platforms
    /// it represents the nominal "fixed" value.
    pub fn current(&self) -> units::Frequency {
        self.as_ref().current()
    }

    /// Minimal CPU frequency.
    ///
    /// ## Returns
    ///
    /// Returns `None` if value can't be determined.
    pub fn min(&self) -> Option<units::Frequency> {
        self.as_ref().min()
    }

    /// Maximal CPU frequency.
    ///
    /// ## Returns
    ///
    /// Returns `None` if value can't be determined.
    pub fn max(&self) -> Option<units::Frequency> {
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
/// [CpuFrequency]: ./struct.CpuFrequency.html
pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    sys::frequency().map_ok(Into::into)
}
