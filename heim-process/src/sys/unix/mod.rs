use std::io;
use std::ptr;
use std::time::Duration;

use heim_common::prelude::StreamExt as _;
use heim_common::Error;
use heim_runtime as rt;

use crate::os::unix::Signal;
use crate::{Pid, ProcessError, ProcessResult};

mod env;

pub use self::env::{Environment, EnvironmentIter, IntoEnvironmentIter};

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
#[allow(unused)] // macos trigger
pub async fn pid_wait(pid: Pid) -> ProcessResult<()> {
    // `waitpid` might block indefinitely,
    // we need to handle that
    let waited = rt::task::spawn_blocking(move || {
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
    if let Ok(Ok(..)) = waited {
        return Ok(());
    }
    // If either task failed or `waitpid` failed,
    // it could mean that either `pid` is not our child
    // or there is no such a process at all.
    // Same to `psutil`, there is nothing left to do,
    // except for a naive checking for pid existence in a loop

    let interval = rt::time::interval(Duration::from_millis(40));
    rt::pin!(interval);
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
