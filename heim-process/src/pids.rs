use heim_common::prelude::{Result, Stream};

use crate::{sys, Pid, ProcessResult};

/// Returns a stream over the [Pid]s of the processes currently running in the system.
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// [Pid]: type.Pid.html
pub async fn pids() -> Result<impl Stream<Item = Result<Pid>>> {
    sys::pids().await
}

/// Checks if the process with given `pid` exists.
pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    sys::pid_exists(pid).await
}
