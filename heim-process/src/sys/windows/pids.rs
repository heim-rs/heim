use std::result;

use heim_common::prelude::*;

use winapi::shared::winerror;
use winapi::um::minwinbase;

use super::bindings;
use crate::{Pid, ProcessError};

pub fn pids() -> impl Stream<Item = result::Result<Pid, ProcessError>> {
    future::lazy(|_| {
        let pids = bindings::pids()?;

        Ok(stream::iter(pids).map(Ok))
    })
    .try_flatten_stream()
    .map_ok(Pid::from)
}

pub fn pid_exists(pid: Pid) -> impl Future<Output = result::Result<bool, ProcessError>> {
    future::lazy(move |_| {
        // Special case for "System Idle Process"
        if pid == 0 {
            return Ok(true);
        }

        let process = match bindings::ProcessHandle::query_limited_info(pid) {
            Ok(process) => process,
            // Means that there is no such process
            Err(ref e) if e.raw_os_error() == Some(winerror::ERROR_INVALID_PARAMETER as i32) => {
                return Ok(false)
            }
            // Process exists, but we do not have an access to it
            Err(ref e) if e.raw_os_error() == Some(winerror::ERROR_ACCESS_DENIED as i32) => {
                return Ok(true)
            }
            Err(e) => return Err(ProcessError::from(Error::from(e))),
        };

        match process.exit_code() {
            // TODO: Same as `psutil` this line is prone to error,
            // if the process had exited with code equal to `STILL_ACTIVE`
            Ok(code) if code == minwinbase::STILL_ACTIVE => Ok(true),
            Err(ref e) if e.raw_os_error() == Some(winerror::ERROR_ACCESS_DENIED as i32) => {
                Ok(true)
            }
            Err(e) => Err(e.into()),
            Ok(..) => {
                // Falling back to checking if pid is in list of running pids
                let pids = bindings::pids().map_err(Error::from)?;

                Ok(pids.contains(&pid))
            }
        }
    })
}
