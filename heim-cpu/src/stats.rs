use std::fmt;

use crate::sys;

use heim_common::prelude::*;

/// CPU statistics.
#[derive(heim_derive::ImplWrap)]
pub struct CpuStats(sys::CpuStats);

// TODO: Custom Debug implementation

impl CpuStats {
    /// Returns number of context switches (voluntary + involuntary) since system boot.
    pub fn ctx_switches(&self) -> u64 {
        self.as_ref().ctx_switches()
    }

    /// Returns number of interrupts since system boot.
    pub fn interrupts(&self) -> u64 {
        self.as_ref().interrupts()
    }

    // TODO: Move to OsExt
    /// Returns number of software interrupts since boot.
    pub fn soft_interrupts(&self) -> u64 {
        self.as_ref().soft_interrupts()
    }
}

impl fmt::Debug for CpuStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuStats")
            .field("ctx_switches", &self.ctx_switches())
            .field("interrupts", &self.interrupts())
            .finish()
    }
}

/// Returns future which will resolve into [CpuStats].
pub fn stats() -> impl Future<Item = CpuStats, Error = Error> {
    sys::stats().map(Into::into)
}
