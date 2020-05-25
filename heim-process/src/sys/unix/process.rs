use std::io;
use std::ptr;
use std::time::Duration;

use heim_common::prelude::StreamExt as _;
use heim_common::Error;
use heim_runtime as rt;

use super::bindings::{errno, set_errno};
use crate::os::unix::Signal;
use crate::{Pid, ProcessError, ProcessResult};

#[cfg(all(target_os = "linux", not(target_env = "musl")))]
#[allow(trivial_numeric_casts)]
const PRIO_PROCESS: libc::c_uint = libc::PRIO_PROCESS as libc::c_uint;
#[cfg(any(all(target_os = "linux", target_env = "musl"), target_os = "macos"))]
#[allow(trivial_numeric_casts)]
const PRIO_PROCESS: libc::c_int = libc::PRIO_PROCESS as libc::c_int;

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

/// Wait for the process termination.
pub async fn pid_wait(pid: Pid) -> ProcessResult<()> {
    // `waitpid` might block indefinitely,
    // we need to handle that
    let waited = rt::spawn_blocking(move || {
        let result = unsafe { libc::waitpid(pid, ptr::null_mut(), 0) };
        if result == -1 {
            // Do not care about the error type at this point
            Err(())
        } else {
            Ok(())
        }
    })
    .await;

    // Task finished correctly and `waitpid` succeeded too
    if waited.is_ok() {
        return Ok(());
    }
    // If either task failed or `waitpid` failed,
    // it could mean that either `pid` is not our child
    // or there is no such a process at all.
    // Same to `psutil`, there is nothing left to do,
    // except for a naive checking for pid existence in a loop

    let interval = rt::time::interval(Duration::from_millis(40));
    futures::pin_mut!(interval);
    while let Some(..) = interval.next().await {
        if !pid_exists(pid) {
            return Ok(());
        }
    }

    // Logically speaking, this branch is unreachable, as `time::interval`
    // is infinite, but in order to make compiler happy,
    // let's show some error
    let e = Error::from(io::Error::from(io::ErrorKind::Other))
        .with_message("Unable to await for process termination");
    Err(e.into())
}

/// Safe wrapper for POSIX `getpriority`
pub fn pid_priority(pid: Pid) -> ProcessResult<libc::c_int> {
    // Since `getpriority()` can legitimately return the value `-1`,
    // it is necessary to clear the external variable `errno` prior to the call,
    // then check it afterward to determine if `-1` is an error or a legitimate value.
    set_errno(0);
    let result = unsafe { libc::getpriority(PRIO_PROCESS, pid as libc::id_t) };

    match errno() {
        0 => Ok(result),
        libc::ESRCH => Err(ProcessError::NoSuchProcess(pid)),
        libc::EACCES | libc::EPERM => Err(ProcessError::AccessDenied(pid)),
        other => Err(Error::from_raw_os_error(other)
            .with_ffi("getpriority")
            .into()),
    }
}

/// Safe wrapper for POSIX `setpriority`
pub fn pid_setpriority(pid: Pid, value: i32) -> ProcessResult<()> {
    let result = unsafe { libc::setpriority(PRIO_PROCESS, pid as libc::id_t, value) };
    if result == 0 {
        Ok(())
    } else {
        Err(Error::last_os_error().into())
    }
}
