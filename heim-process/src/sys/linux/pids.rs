use heim_common::prelude::*;
use heim_common::utils::fs;

use crate::Pid;

pub fn pids() -> impl Stream<Item = Result<Pid>> {
    fs::read_dir("/proc")
        .try_filter_map(|entry| {
            let res = match entry.file_name().to_str() {
                Some(name) => name.parse::<Pid>().ok(),
                None => None,
            };

            future::ok(res)
        })
}

pub fn pid_exists(pid: Pid) -> impl Future<Output = Result<bool>> {
    let path = format!("/proc/{}", pid);

    utils::fs::path_exists(path).map(Ok)
}
