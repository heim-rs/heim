use std::fmt;

use heim_common::units::{information, Information};
use winapi::um::psapi;

pub struct Memory(psapi::PROCESS_MEMORY_COUNTERS_EX);

impl Memory {
    pub fn rss(&self) -> Information {
        // TODO: Possible truncation from `usize` to `u64`
        Information::new::<information::byte>(self.0.WorkingSetSize as u64)
    }

    pub fn vms(&self) -> Information {
        // TODO: Possible truncation from `usize` to `u64`
        Information::new::<information::byte>(self.0.PagefileUsage as u64)
    }

    pub fn page_faults(&self) -> u32 {
        self.0.PageFaultCount
    }

    pub fn peak_working_set_size(&self) -> usize {
        self.0.PeakWorkingSetSize
    }

    pub fn working_set_size(&self) -> usize {
        self.0.WorkingSetSize
    }

    pub fn quota_peak_paged_pool_usage(&self) -> usize {
        self.0.QuotaPeakPagedPoolUsage
    }

    pub fn quota_paged_pool_usage(&self) -> usize {
        self.0.QuotaPagedPoolUsage
    }

    pub fn quota_peak_non_paged_pool_usage(&self) -> usize {
        self.0.QuotaPeakNonPagedPoolUsage
    }

    pub fn quota_non_paged_pool_usage(&self) -> usize {
        self.0.QuotaNonPagedPoolUsage
    }

    pub fn pagefile_usage(&self) -> usize {
        self.0.PagefileUsage
    }

    pub fn peak_pagefile_usage(&self) -> usize {
        self.0.PeakPagefileUsage
    }

    pub fn private_usage(&self) -> usize {
        self.0.PrivateUsage
    }
}

impl From<psapi::PROCESS_MEMORY_COUNTERS_EX> for Memory {
    fn from(counters: psapi::PROCESS_MEMORY_COUNTERS_EX) -> Memory {
        Memory(counters)
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("rss", &self.rss())
            .field("vms", &self.vms())
            .finish()
    }
}
