use std::fmt;

use heim_common::prelude::wrap;
use heim_common::units::Information;

use crate::sys;

/// Memory information about the process.
///
/// See os-specific extensions also.
pub struct Memory(sys::Memory);

wrap!(Memory, sys::Memory);

impl Memory {
    /// Returns resident set size, amount of non-swapped physical memory used by the process.
    pub fn rss(&self) -> Information {
        self.as_ref().rss()
    }

    /// Returns virtual memory size, total amount of memory used by the process.
    pub fn vms(&self) -> Information {
        self.as_ref().vms()
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
