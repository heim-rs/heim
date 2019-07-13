use heim_common::prelude::{Future, Stream};

use crate::{sys, Pid, ProcessError};

/// Returns stream which yields [Pid]s of processes currently running in the system.
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// [Pid]: type.Pid.html
pub fn pids() -> impl Stream<Item = Result<Pid, ProcessError>> {
    sys::pids()
}

/// Returns future which checks if process with passed `pid` is exists.
pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool, ProcessError>> {
    sys::pid_exists(pid)
}
