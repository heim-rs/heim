use heim_common::prelude::{Stream, TryStreamExt, Result};
use heim_rt as rt;

use crate::sys::unix;
use crate::{Pid, ProcessResult};

pub async fn pids() -> Result<impl Stream<Item = Result<Pid>>> {
    let entries = rt::fs::read_dir("/proc").await?;

    let stream = entries
        .map_err(Into::into)
        .try_filter_map(|entry| async move {
            let res = match entry.file_name().to_str() {
                Some(name) => name.parse::<Pid>().ok(),
                None => None,
            };

            Ok(res)
        });

    Ok(stream)
}

pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    Ok(unix::pid_exists(pid))
}
