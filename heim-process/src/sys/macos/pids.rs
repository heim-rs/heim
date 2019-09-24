use heim_common::prelude::{future, stream, Future, Stream, TryFutureExt};

use super::bindings;
use crate::sys::unix;
use crate::{Pid, ProcessError};

pub fn pids() -> impl Stream<Item = Result<Pid, ProcessError>> {
    future::lazy(|_| {
        // `kinfo_proc` is not `Send`-able, so it would not be possible
        // later to send it between threads (it's full of raw pointers),
        // so for MVP we are just going to collect all the pids in-place.
        let pids = bindings::processes()?
            .into_iter()
            .map(|proc| Ok(proc.kp_proc.p_pid))
            .collect::<Vec<_>>();

        Ok(stream::iter(pids))
    })
    .try_flatten_stream()
}

pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool, ProcessError>> {
    future::ok(unix::pid_exists(pid))
}
