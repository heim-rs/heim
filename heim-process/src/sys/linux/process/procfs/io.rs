use std::io;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::iter::TryIterator;
use heim_common::Pid;
use heim_runtime::fs;

use crate::os::linux::IoCounters;
use crate::{ProcessError, ProcessResult};

impl FromStr for IoCounters {
    type Err = Error2;

    fn from_str(s: &str) -> Result2<Self> {
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
                other => {
                    let inner = io::Error::from(io::ErrorKind::InvalidData);

                    return Err(
                        Error2::from(inner).with_message(format!("Unknown field {}", other))
                    );
                }
            };

            *field = parts.try_next()?.parse::<u64>()?;
        }

        Ok(counters)
    }
}

pub async fn io(pid: Pid) -> ProcessResult<IoCounters> {
    let path = format!("/proc/{}/io", pid);
    match fs::read_into::<_, _, Error2>(path).await {
        Ok(counters) => Ok(counters),
        // TODO: Use error kind instead
        Err(e) if e.raw_os_error() == Some(libc::EACCES) => Err(ProcessError::AccessDenied(pid)),
        Err(e) => Err(e.into()),
    }
}
