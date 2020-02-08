use std::fmt;

use heim_common::prelude::wrap;
use heim_common::units::Time;

use crate::sys;

/// Accumulated CPU time for specific process.
pub struct CpuTime(sys::CpuTime);

wrap!(CpuTime, sys::CpuTime);

impl CpuTime {
    /// Returns amount of CPU time spent in user mode within the process.
    pub fn user(&self) -> Time {
        self.as_ref().user()
    }

    /// Returns amount of CPU time spent in kernel within the process.
    pub fn system(&self) -> Time {
        self.as_ref().system()
    }
}

impl fmt::Debug for CpuTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuTime")
            .field("user", &self.user())
            .field("system", &self.system())
            .finish()
    }
}
