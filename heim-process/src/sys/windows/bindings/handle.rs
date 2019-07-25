use std::io::{Result, Error};
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::{winnt, processthreadsapi, handleapi, winbase};
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::ctypes::wchar_t;

use crate::Pid;

#[derive(Debug)]
pub struct ProcessHandle(winnt::HANDLE);

impl ProcessHandle {
    pub fn query_info(pid: Pid) -> Result<ProcessHandle> {
        let handle = unsafe {
            processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ,
                0,
                pid
            )
        };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ProcessHandle(handle))
        }
    }

    pub fn query_limited_info(pid: Pid) -> Result<ProcessHandle> {
        let handle = unsafe {
            processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_LIMITED_INFORMATION | winnt::PROCESS_VM_READ,
                0,
                pid
            )
        };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ProcessHandle(handle))
        }
    }

    pub fn exit_code(&self) -> Result<DWORD> {
        let mut code: DWORD = 0;

        let result = unsafe {
            processthreadsapi::GetExitCodeProcess(self.0, &mut code)
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(code)
        }
    }

    pub fn exe(&self) -> Result<PathBuf> {
        let mut buffer: [wchar_t; MAX_PATH] = [0; MAX_PATH];
        let mut size = MAX_PATH as DWORD;

        let result = unsafe {
            winbase::QueryFullProcessImageNameW(
                self.0,
                0,
                buffer.as_mut_ptr(),
                &mut size,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(OsString::from_wide(&buffer[..(size as usize)]).into())
        }
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        let result = unsafe {
            handleapi::CloseHandle(self.0)
        };

        debug_assert!(result != 0);
    }
}
