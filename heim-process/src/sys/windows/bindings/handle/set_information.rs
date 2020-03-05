//! Process handle variant for setting information for process.

use std::marker::PhantomData;

use winapi::shared::minwindef::DWORD;
use winapi::um::{processthreadsapi, winnt};

use heim_common::{Error, Result};

use super::{ProcessHandle, ProcessHandlePermissions};
use crate::os::windows::Priority;
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug)]
pub struct SetInformation;
impl ProcessHandlePermissions for SetInformation {
    const ACCESS: DWORD = winnt::PROCESS_SET_INFORMATION;
}

impl ProcessHandle<SetInformation> {
    pub fn for_set_information(pid: Pid) -> ProcessResult<ProcessHandle<SetInformation>> {
        if pid == 0 {
            return Err(ProcessError::AccessDenied(pid));
        }

        let handle = SetInformation::open(pid)?;

        Ok(ProcessHandle {
            handle,
            pid,
            _type: PhantomData,
        })
    }

    /// Set process priority.
    ///
    /// `priority` method is located at `ProcessHandle<QueryLimitedInformation>` impl
    pub fn set_priority(&self, value: Priority) -> Result<()> {
        let result = unsafe { processthreadsapi::SetPriorityClass(*self.handle, value.into()) };
        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
