use heim_common::prelude::Stream;

use crate::{sys, Pid, ProcessResult};

/// Returns a stream over the [Pid]s of the processes currently running in the system.
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// [Pid]: type.Pid.html
pub fn pids() -> impl Stream<Item = ProcessResult<Pid>> {
    sys::pids()
}

/// Checks if the process with given `pid` exists.
pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    sys::pid_exists(pid).await
}
