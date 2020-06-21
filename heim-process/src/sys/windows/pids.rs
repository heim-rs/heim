use heim_common::prelude::*;

use winapi::um::minwinbase;

use super::bindings;
use crate::{Pid, ProcessError, ProcessResult};

#[allow(clippy::useless_conversion)]
pub async fn pids() -> Result<impl Stream<Item = Result<Pid>>> {
    let pids = bindings::pids()?.into_iter().map(|pid| Ok(Pid::from(pid)));

    Ok(stream::iter(pids))
}

pub async fn pid_exists(pid: Pid) -> ProcessResult<bool> {
    // Special case for "System Idle Process"
    if pid == 0 {
        return Ok(true);
    }

    let process = match bindings::ProcessHandle::query_limited_info(pid) {
        Ok(process) => process,
        // Well, nothing do to here anymore
        Err(ProcessError::NoSuchProcess(..)) => return Ok(false),
        // Process exists, but we do not have an access to it
        Err(ProcessError::AccessDenied(..)) => return Ok(true),
        Err(e) => return Err(e),
    };

    match process.exit_code() {
        // TODO: Same as `psutil` this line is prone to error,
        // if the process had exited with code equal to `STILL_ACTIVE`
        Ok(code) if code == minwinbase::STILL_ACTIVE => Ok(true),
        Ok(..) => {
            // Falling back to checking if pid is in list of running pids
            let pids = bindings::pids().map_err(Error::from)?;

            Ok(pids.contains(&pid))
        }
        Err(e) => Err(e),
    }
}
