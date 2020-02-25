//! Process handle variant for process suspend or resume.

use std::marker::PhantomData;

use ntapi::ntpsapi;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntstatus;
use winapi::um::winnt;

use heim_common::{Error, Result};

use super::{ProcessHandle, ProcessHandlePermissions};
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug)]
pub struct SuspendResume;
impl ProcessHandlePermissions for SuspendResume {
    const ACCESS: DWORD = winnt::PROCESS_SUSPEND_RESUME;
}

impl ProcessHandle<SuspendResume> {
    pub fn for_suspend_resume(pid: Pid) -> ProcessResult<ProcessHandle<SuspendResume>> {
        if pid == 0 {
            return Err(ProcessError::AccessDenied(pid));
        }

        let handle = SuspendResume::open(pid)?;

        Ok(ProcessHandle {
            handle,
            pid,
            _type: PhantomData,
        })
    }

    pub fn suspend(&self) -> Result<()> {
        let result = unsafe { ntpsapi::NtSuspendProcess(*self.handle) };

        if result != ntstatus::STATUS_SUCCESS {
            Err(Error::last_os_error().with_ffi("NtSuspendProcess"))
        } else {
            Ok(())
        }
    }

    pub fn resume(&self) -> Result<()> {
        let result = unsafe { ntpsapi::NtResumeProcess(*self.handle) };

        if result != ntstatus::STATUS_SUCCESS {
            Err(Error::last_os_error().with_ffi("NtResumeProcess"))
        } else {
            Ok(())
        }
    }
}
