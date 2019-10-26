//! Linux-specific extensions.
//!
//! Available only for `cfg(target_os = "linux")`

use heim_common::prelude::*;
use heim_common::Pid;

use crate::{sys, IoCounters};

/// Linux-specific extension for [IoCounters].
///
/// [IoCounters]: ../../struct.IoCounters.html
pub trait IoCountersExt {
    /// Returns packets amount which were dropped while sending them.
    fn drop_sent(&self) -> u64;
}

#[cfg(target_os = "linux")]
impl IoCountersExt for crate::IoCounters {
    fn drop_sent(&self) -> u64 {
        self.as_ref().drop_sent()
    }
}

/// Returns stream which yield [IO counters] for each network interface for process with given `pid`.
///
/// **MUST** be used as `process::Process::net_io_counters()`
///
/// ## Compatibility
///
/// Implemented only for Linux for now. For other platforms will return an empty stream.
#[doc(hidden)]
pub fn io_counters_for_pid(pid: Pid) -> impl Stream<Item = Result<IoCounters>> {
    sys::io_counters_for_pid(pid).map_ok(Into::into)
}
