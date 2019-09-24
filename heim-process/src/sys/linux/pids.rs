use heim_common::prelude::{future, Future, Stream, TryStreamExt};
use heim_runtime::fs;

use crate::sys::unix;
use crate::{Pid, ProcessError};

pub fn pids() -> impl Stream<Item = Result<Pid, ProcessError>> {
    fs::read_dir("/proc")
        .map_err(From::from)
        .try_filter_map(|entry| {
            let res = match entry.file_name().to_str() {
                Some(name) => name.parse::<Pid>().ok(),
                None => None,
            };

            future::ok(res)
        })
}

pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool, ProcessError>> {
    future::ok(unix::pid_exists(pid))
}
