use crate::sys;

use heim_common::prelude::*;

/// CPU statistics.
#[derive(Debug, heim_derive::ImplWrap)]
pub struct CpuStats(sys::CpuStats);

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

/// Returns future which will resolve into [CpuStats].
pub fn stats() -> impl Future<Item = CpuStats, Error = Error> {
    sys::stats().map(Into::into)
}
