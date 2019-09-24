use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::iter::TryIterator;
use heim_common::Pid;
use heim_runtime::fs;

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
                other => return Err(Error::incompatible(format!("Unknown field {}", other))),
            };

            *field = parts.try_next()?.parse::<u64>()?;
        }

        Ok(counters)
    }
}

pub fn io(pid: Pid) -> impl Future<Output = ProcessResult<IoCounters>> {
    fs::read_into(format!("/proc/{}/io", pid)).map_err(move |e: Error| {
        match e.raw_os_error() {
            // TODO: It is not possible to get `::std::io::ErrorKind` from the `heim::Error`
            Some(libc::EACCES) => ProcessError::AccessDenied(pid),
            _ => e.into(),
        }
    })
}
