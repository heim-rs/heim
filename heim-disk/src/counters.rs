use std::ffi::OsStr;
use std::fmt;

use heim_common::prelude::*;
use heim_common::units::Information;

use crate::sys;

/// Disk I/O counters.
///
/// ## Compatibility
///
/// See [os]-specific extension traits also.
///
/// On some systems such a Linux the numbers returned may overflow and wrap.
/// Contrary to `psutil` behavior, at the moment `heim` will not automatically
/// handle these cases and returned values might wrap.
///
/// [os]: ./os/index.html
#[derive(heim_derive::ImplWrap)]
pub struct IoCounters(sys::IoCounters);

impl IoCounters {
    /// Returns disk device name.
    pub fn device_name(&self) -> &OsStr {
        self.as_ref().device_name()
    }

    /// Returns number of reads.
    pub fn read_count(&self) -> u64 {
        self.as_ref().read_count()
    }

    /// Returns number of writes.
    pub fn write_count(&self) -> u64 {
        self.as_ref().write_count()
    }

    /// Returns number of bytes read.
    pub fn read_bytes(&self) -> Information {
        self.as_ref().read_bytes()
    }

    /// Returns number of bytes written.
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

/// Returns stream which will yield [IO counters] for all disks available in system.
///
/// ## Compatibility
///
/// Same to similar tools, on Windows it may be necessary to issue `diskperf -y` command
/// from `cmd.exe` first in order to enable IO counters.
///
/// ## Examples
///
/// ```rust
/// # #![feature(async_await)]
/// # use heim_common::prelude::*;
/// # use heim_common::Result;
/// # use heim_disk::io_counters;
/// #
/// # #[runtime::main]
/// # async fn main() -> Result<()> {
/// let mut counters_stream = io_counters();
/// while let Some(counter) = counters_stream.next().await {
///    dbg!(counter?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// [IO counters]: struct.IoCounters.html
pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters().map_ok(Into::into)
}

/// Returns future which will resolve into [IO counters]
/// for each physical disk installed on the system.
///
/// ## Examples
///
/// ```rust
/// # #![feature(async_await)]
/// # use heim_common::prelude::*;
/// # use heim_common::Result;
/// # use heim_disk::io_counters_physical;
/// #
/// # #[runtime::main]
/// # async fn main() -> Result<()> {
/// let mut counters_stream = io_counters_physical();
/// while let Some(counter) = counters_stream.next().await {
///    dbg!(counter?);
/// }
/// # Ok(())
/// # }
/// ```
///
/// [IO counters]: struct.IoCounters.html
pub fn io_counters_physical() -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters_physical().map_ok(Into::into)
}
