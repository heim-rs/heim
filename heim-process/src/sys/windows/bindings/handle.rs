use std::mem;
use std::io::{Result, Error};
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::{winnt, processthreadsapi, handleapi, winbase, psapi};
use winapi::shared::minwindef::{DWORD, MAX_PATH, FILETIME};
use winapi::ctypes::wchar_t;

use heim_common::sys::windows::IntoTime;

use super::super::process::CpuTime;
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

    pub fn memory(&self) -> Result<psapi::PROCESS_MEMORY_COUNTERS_EX> {
        let mut counters = mem::MaybeUninit::<psapi::PROCESS_MEMORY_COUNTERS_EX>::uninit();

        let result = unsafe {
            psapi::GetProcessMemoryInfo(
                self.0,
                // Tricking the type checker,
                // as the `winapi`' GetProcessMemoryInfo expects `PROCESS_MEMORY_COUNTERS`,
                // not the `PROCESS_MEMORY_COUNTERS_EX`
                counters.as_mut_ptr() as *mut psapi::PROCESS_MEMORY_COUNTERS,
                mem::size_of::<psapi::PROCESS_MEMORY_COUNTERS_EX>() as DWORD,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            unsafe {
                Ok(counters.assume_init())
            }
        }
    }

    pub fn cpu_time(&self) -> Result<CpuTime> {
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        let result = unsafe {
            processthreadsapi::GetProcessTimes(
                self.0,
                &mut creation,
                &mut exit,
                &mut kernel,
                &mut user,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(CpuTime {
                user: user.into_time(),
                kernel: kernel.into_time(),
            })
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
