//! Process handle variant used for process termination.

use std::marker::PhantomData;

use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror;
use winapi::um::{processthreadsapi, winnt};

use heim_common::Error;

use super::{ProcessHandle, ProcessHandlePermissions};
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug)]
pub struct Termination;
impl ProcessHandlePermissions for Termination {
    const ACCESS: DWORD = winnt::PROCESS_TERMINATE;
}

impl ProcessHandle<Termination> {
    pub fn for_termination(pid: Pid) -> ProcessResult<ProcessHandle<Termination>> {
        if pid == 0 {
            return Err(ProcessError::AccessDenied(pid));
        }

        let handle = Termination::open(pid)?;

        Ok(ProcessHandle {
            handle,
            pid,
            _type: PhantomData,
        })
    }

    /// ERROR_INVALID_PARAMETER should be considered as a `NoSuchProcess` later
    pub fn terminate(&self) -> ProcessResult<()> {
        let result = unsafe {
            processthreadsapi::TerminateProcess(
                *self.handle,
                // This is going to be the code with which the process will exit
                libc::SIGTERM as u32,
            )
        };
        // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess

        if result == 0 {
            match Error::last_os_error().with_ffi("TerminateProcess") {
                // TODO: `psutil` ignores permission errors and considers the target process
                // to be dead already.
                // See: https://github.com/giampaolo/psutil/issues/1099
                // It seems kinda shady, so the behavior should be checked.
                e if e.raw_os_error() == Some(winerror::ERROR_ACCESS_DENIED as i32) => Ok(()),
                e => Err(e.into()),
            }
        } else {
            Ok(())
        }
    }
}
