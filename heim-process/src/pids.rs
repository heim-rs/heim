use heim_common::prelude::*;

use crate::{sys, Pid};

/// Returns stream which yields [Pid].
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// [Pid]: type.Pid.html
pub fn pids() -> impl Stream<Item = Result<Pid>> {
    sys::pids()
}

/// Returns future which checks is process with passed `pid` is exists.
pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool>> {
    sys::pid_exists(pid)
}
