use heim_common::prelude::*;

use crate::Pid;

pub fn pids() -> impl Stream<Item = Result<Pid>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/46
    stream::iter(vec![])
}

pub fn pid_exists(_pid: Pid) -> impl Future<Output = Result<bool>> {
    // TODO: Stub
    future::ok(false)
}
