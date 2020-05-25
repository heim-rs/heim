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
#[cfg(target_os = "linux")]
pub async fn io_counters_for_pid(pid: Pid) -> Result<impl Stream<Item = Result<IoCounters>>> {
    let inner = sys::io_counters_for_pid(pid).await?;

    Ok(inner.map_ok(Into::into))
}
