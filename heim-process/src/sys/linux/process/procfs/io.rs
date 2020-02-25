use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::iter::TryIterator;
use heim_common::Pid;
use heim_runtime as rt;

use crate::os::linux::IoCounters;
use crate::{ProcessError, ProcessResult};

impl FromStr for IoCounters {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut counters = IoCounters::default();
        for line in s.lines() {
            let mut parts = line.split_ascii_whitespace();
            let field = match parts.try_next()? {
                "rchar:" => &mut counters.rchar,
                "wchar:" => &mut counters.wchar,
                "syscr:" => &mut counters.syscr,
                "syscw:" => &mut counters.syscw,
                "read_bytes:" => &mut counters.read_bytes,
                "write_bytes:" => &mut counters.write_bytes,
                "cancelled_write_bytes:" => &mut counters.cancelled_write_bytes,
                _ => continue,
            };

            *field = parts.try_next()?.parse::<u64>()?;
        }

        Ok(counters)
    }
}

pub async fn io(pid: Pid) -> ProcessResult<IoCounters> {
    let path = format!("/proc/{}/io", pid);
    match rt::fs::read_to_string(path).await {
        Ok(contents) => IoCounters::from_str(&contents).map_err(Into::into),
        Err(e) if e.raw_os_error() == Some(libc::EACCES) => Err(ProcessError::AccessDenied(pid)),
        Err(e) => Err(e.into()),
    }
}
