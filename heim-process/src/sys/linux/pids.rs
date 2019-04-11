use heim_common::prelude::*;

use crate::Pid;

pub fn pids() -> impl Stream<Item=Pid, Error=Error> {
    tokio::fs::read_dir("/proc/")
        .flatten_stream()
        .map_err(Error::from)
        .filter_map(|entry| {
            match entry.file_name().to_str() {
                Some(filename) => filename.parse::<Pid>().ok(),
                None => None,
            }
        })
}

pub fn pid_exists(pid: Pid) -> impl Future<Item=bool, Error=Error> {
    let path = format!("/proc/{}", pid);

    utils::fs::path_exists(path)
}
