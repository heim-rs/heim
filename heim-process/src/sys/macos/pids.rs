use heim_common::prelude::{future, stream, Stream, TryFutureExt};

use super::wrappers;
use crate::sys::unix;
use crate::{Pid, ProcessResult};

pub fn pids() -> impl Stream<Item = ProcessResult<Pid>> {
    future::lazy(|_| {
        // `kinfo_proc` is not `Send`-able, so it would not be possible
        // later to send it between threads (it's full of raw pointers),
        // so for MVP we are just going to collect all the pids in-place.
        // TODO: Could we Pin it maybe?
        let pids = wrappers::processes()?
            .into_iter()
            .map(|proc| Ok(proc.kp_proc.p_pid))
            .collect::<Vec<_>>();

        Ok(stream::iter(pids))
    })
    .try_flatten_stream()
}

pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    Ok(unix::pid_exists(pid))
}
