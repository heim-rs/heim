use heim_common::prelude::{Future, Stream};

use crate::{sys, Pid, ProcessError};

/// Returns stream which yields [Pid].
///
/// Consequent calls are not guaranteed to return pids in the same order.
///
/// ## Compatibility
///
/// For Windows this function will always return an empty stream,
/// since it is not implemented yet,
/// see the [related issue](https://github.com/heim-rs/heim/issues/46)
///
/// [Pid]: type.Pid.html
pub fn pids() -> impl Stream<Item = Result<Pid, ProcessError>> {
    sys::pids()
}

/// Returns future which checks is process with passed `pid` is exists.
///
/// ## Compatibility
///
/// For macOS and Windows this function will always return `Ok(false)`,
/// since it is not implemented yet.
pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool, ProcessError>> {
    sys::pid_exists(pid)
}
