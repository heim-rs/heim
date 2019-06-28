use heim_common::prelude::*;

use crate::Pid;
use super::bindings::processes;

pub fn pids() -> impl Stream<Item = Result<Pid>> {
    future::lazy(|_| {
        // `kinfo_proc` is not `Send`-able, so it would not be possible
        // later to send it between threads (it's full of raw pointers),
        // so for MVP we are just going to collect all the pids in-place.
        let pids = processes()?.into_iter()
            .map(|proc| Ok(proc.kp_proc.p_pid))
            .collect::<Vec<_>>();

        Ok(stream::iter(pids))
    })
    .try_flatten_stream()
}

pub fn pid_exists(_pid: Pid) -> impl Future<Output = Result<bool>> {
    // TODO: Stub
    future::ok(false)
}
