use std::io;
use std::marker::PhantomData;

use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror;
use winapi::um::processthreadsapi;

use heim_common::sys::windows::Handle;
use heim_common::Error;

use crate::{Pid, ProcessError, ProcessResult};

mod limited_info;
mod set_information;
mod suspend;
mod termination;

pub use limited_info::QueryLimitedInformation;
pub use suspend::SuspendResume;
pub use termination::Termination;

pub trait ProcessHandlePermissions {
    /// Desired access flags for `OpenProcess` function.
    const ACCESS: DWORD;

    /// Open the process handle with `Self::ACCESS` permissions.
    fn open(pid: Pid) -> ProcessResult<Handle> {
        let handle = unsafe { processthreadsapi::OpenProcess(Self::ACCESS, 0, pid) };

        if handle.is_null() {
            let e = Error::last_os_error().with_ffi("OpenProcess");
            match e.as_inner() {
                err if err.kind() == io::ErrorKind::PermissionDenied => {
                    Err(ProcessError::AccessDenied(pid))
                }

                // Notable error which might be returned here is
                // `ERROR_INVALID_PARAMETER` ("The parameter is incorrect").
                // Might mean that we are querying process with pid 0 (System Process)
                err if pid == 0
                    && err.raw_os_error() == Some(winerror::ERROR_INVALID_PARAMETER as i32) =>
                {
                    Err(ProcessError::AccessDenied(pid))
                }
                // For other processes it is assumed that process is gone
                err if err.raw_os_error() == Some(winerror::ERROR_INVALID_PARAMETER as i32) => {
                    Err(ProcessError::NoSuchProcess(pid))
                }

                _ => Err(ProcessError::from(e)),
            }
        } else {
            Ok(Handle::new(handle))
        }
    }
}

#[derive(Debug)]
pub struct ProcessHandle<T> {
    handle: Handle,
    pid: Pid,
    _type: PhantomData<T>,
}
