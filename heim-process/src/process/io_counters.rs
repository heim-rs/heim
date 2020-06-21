use heim_common::prelude::wrap;
use heim_common::units::Information;
use std::fmt;

use crate::sys;

/// IO information about the process.
///
/// See os-specific extensions also.
pub struct IoCounters(sys::IoCounters);

wrap!(IoCounters, sys::IoCounters);

impl IoCounters {
    /// Attempt to count the number of bytes which this process really did cause to
    /// be fetched from the storage layer.
    pub fn bytes_read(&self) -> Information {
        self.as_ref().bytes_read()
    }

    /// Attempt to count the number of bytes which this process caused to be sent to
    /// the storage layer.
    pub fn bytes_written(&self) -> Information {
        self.as_ref().bytes_written()
    }
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("bytes_read", &self.bytes_read())
            .field("bytes_written", &self.bytes_written())
            .finish()
    }
}
