use std::io;

use crate::os::unix::Signal;
use crate::{Pid, ProcessError, ProcessResult};

pub fn pid_exists(pid: Pid) -> bool {
    if pid == 0 {
        return true;
    }

    let result = unsafe { libc::kill(pid, 0) };

    if result == 0 {
        true
    } else {
        let e = io::Error::last_os_error();
        match e.raw_os_error() {
            Some(libc::ESRCH) => false,
            Some(libc::EPERM) => true,
            _ => true,
        }
    }
}

pub fn pid_kill(pid: Pid, signal: Signal) -> ProcessResult<()> {
    let result = unsafe { libc::kill(pid, signal.into()) };

    if result == 0 {
        Ok(())
    } else {
        let e = io::Error::last_os_error();
        match e.raw_os_error() {
            Some(libc::ESRCH) => Err(ProcessError::NoSuchProcess(pid)),
            Some(libc::EPERM) => Err(ProcessError::AccessDenied(pid)),
            _ => Err(e.into()),
        }
    }
}
