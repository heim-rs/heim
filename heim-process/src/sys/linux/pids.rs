use heim_common::prelude::{future, Stream, TryStreamExt};
use heim_runtime::fs;

use crate::sys::unix;
use crate::{Pid, ProcessResult};

pub fn pids() -> impl Stream<Item = ProcessResult<Pid>> {
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

pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    Ok(unix::pid_exists(pid))
}
