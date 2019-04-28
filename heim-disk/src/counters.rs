use std::fmt;

use heim_common::prelude::*;
use heim_common::units::iec::u64::Information;

use crate::sys;

/// Disk I/O counters.
///
/// On some systems such a Linux the numbers returned may overflow and wrap.
/// Contrary to `psutil` behavior, at the moment `heim` will not automatically
/// handle these cases and returned values might wrap.
#[derive(heim_derive::ImplWrap)]
pub struct IoCounters(sys::IoCounters);

impl IoCounters {
    pub fn device_name(&self) -> &str {
        self.as_ref().device_name()
    }

    pub fn read_count(&self) -> u64 {
        self.as_ref().read_count()
    }

    pub fn write_count(&self) -> u64 {
        self.as_ref().write_count()
    }

    pub fn read_bytes(&self) -> Information {
        self.as_ref().read_bytes()
    }

    pub fn write_bytes(&self) -> Information {
        self.as_ref().write_bytes()
    }
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("device_name", &self.device_name())
            .field("read_count", &self.read_count())
            .field("write_count", &self.write_count())
            .field("read_bytes", &self.read_bytes())
            .field("write_bytes", &self.write_bytes())
            .finish()
    }
}

/// Returns stream which will yield [IO counters] for disk available in system.
///
/// # Compatibility
///
/// Same to similar tools, on Windows it may be necessary to issue `diskperf -y` command
/// from `cmd.exe` first in order to enable IO counters.
///
/// [IO counters]: struct.IoCounters.html
pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters().map_ok(Into::into)
}

/// Returns future which will resolve into [IO counters]
/// for each physical disk installed on the system.
///
/// [IO counters]: struct.IoCounters.html
pub fn io_counters_physical() -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters_physical().map_ok(Into::into)
}
