//! Linux-specific extensions

/// Reference: https://gitlab.com/procps-ng/procps/blob/master/proc/sysinfo.c
use heim_common::units::{information, Information};

use crate::Memory;

/// Linux-specific extension to [`Memory`]
pub trait MemoryExt {
    /// The amount of physical RAM used.
    ///
    /// It is designed for informational purposes only and might vary vastly
    /// from platform to platform.
    fn used(&self) -> Information;

    /// The amount of physical RAM used for file buffers.
    fn buffers(&self) -> Information;

    /// The amount of physical RAM used as cache memory.
    fn cached(&self) -> Information;

    /// The amount of memory that may be simultaneously accessed by multiple processes.
    fn shared(&self) -> Information;

    /// The total amount of buffer or page cache memory, that is in active use.
    ///
    /// This is memory that has been recently used and is usually not reclaimed for other purposes.
    fn active(&self) -> Information;

    ///  The total amount of buffer or page cache memory, that are free and available.
    ///
    /// This is memory that has not been recently used and can be reclaimed for other purposes.
    fn inactive(&self) -> Information;

    /// Memory which is waiting to get written back to the disk.
    fn dirty(&self) -> Information;
}

#[cfg(target_os = "linux")]
impl MemoryExt for Memory {
    fn used(&self) -> Information {
        let inner = self.as_ref();

        let mut used = inner.total() - inner.free() - self.cached() - self.buffers();
        if used <= Information::new::<information::byte>(0) {
            // May be symptomatic of running within a LCX container where such
            // values will be dramatically distorted over those of the host.
            // Source: psutil
            used = inner.total() - inner.free()
        }

        used
    }

    fn buffers(&self) -> Information {
        self.as_ref().buffers()
    }

    fn cached(&self) -> Information {
        self.as_ref().cached()
    }

    fn shared(&self) -> Information {
        self.as_ref().shared()
    }

    fn active(&self) -> Information {
        self.as_ref().active()
    }

    fn inactive(&self) -> Information {
        self.as_ref().inactive()
    }

    fn dirty(&self) -> Information {
        self.as_ref().dirty()
    }
}
