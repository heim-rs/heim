use heim_common::prelude::{stream, future, Stream, Future};

use crate::{Pid, ProcessError};

pub fn pids() -> impl Stream<Item = Result<Pid, ProcessError>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/46
    stream::iter(vec![])
}

pub fn pid_exists(_pid: Pid) -> impl Future<Output = Result<bool, ProcessError>> {
    // TODO: Stub
    future::ok(false)
}
