use std::ffi::OsString;
use std::io::{Error, Result};
use std::marker::PhantomData;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use ntapi::ntpsapi;
use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::{DWORD, FILETIME, MAX_PATH};
use winapi::shared::{ntstatus, winerror};
use winapi::um::{handleapi, processthreadsapi, psapi, winbase, winnt};

use heim_common::sys::IntoTime;
use heim_common::units::{time, Time};

use super::super::process::CpuTime;
use crate::Pid;

pub trait ProcessHandlePermissions {}

#[derive(Debug)]
pub struct QueryLimitedInformation;
impl ProcessHandlePermissions for QueryLimitedInformation {}

#[derive(Debug)]
pub struct Termination;
impl ProcessHandlePermissions for Termination {}

#[derive(Debug)]
pub struct SuspendResume;
impl ProcessHandlePermissions for SuspendResume {}

#[derive(Debug)]
pub struct ProcessHandle<T> {
    handle: winnt::HANDLE,
    _type: PhantomData<T>,
}

impl ProcessHandle<QueryLimitedInformation> {
    // Notable error which might be returned here is
    // `ERROR_INVALID_PARAMETER` ("The parameter is incorrect").
    // Might mean that we are querying process with pid 0 (System Process)
    //
    // Same applies to `Self::query_info`.
    //
    // TODO: Return `ProcessError` from here directly? https://github.com/heim-rs/heim/issues/155
    pub fn query_limited_info(pid: Pid) -> Result<ProcessHandle<QueryLimitedInformation>> {
        let handle = unsafe {
            processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_LIMITED_INFORMATION | winnt::PROCESS_VM_READ,
                0,
                pid,
            )
        };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ProcessHandle {
                handle,
                _type: PhantomData,
            })
        }
    }

    pub fn exit_code(&self) -> Result<DWORD> {
        let mut code: DWORD = 0;

        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess
            processthreadsapi::GetExitCodeProcess(self.handle, &mut code)
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
            winbase::QueryFullProcessImageNameW(self.handle, 0, buffer.as_mut_ptr(), &mut size)
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
                self.handle,
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
            unsafe { Ok(counters.assume_init()) }
        }
    }

    pub fn cpu_time(&self) -> Result<CpuTime> {
        let (_, _, kernel, user) = self.process_times()?;

        Ok(CpuTime {
            user: user.into_time(),
            kernel: kernel.into_time(),
        })
    }

    pub fn create_time(&self) -> Result<Time> {
        let (creation, _, _, _) = self.process_times()?;
        /// Seconds amount between the "Windows epoch" (January 1, 1601)
        /// and the Unix epoch (January 1, 1970).
        // TODO: It would be nice to make it const,
        // as soon as `uom` will mark `Time::new` as a `const fn`
        let unix_epoch_delta = Time::new::<time::second>(11_644_473_600.0);

        Ok(creation.into_time() - unix_epoch_delta)
    }

    fn process_times(&self) -> Result<(FILETIME, FILETIME, FILETIME, FILETIME)> {
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        let result = unsafe {
            processthreadsapi::GetProcessTimes(
                self.handle,
                &mut creation,
                &mut exit,
                &mut kernel,
                &mut user,
            )
        };

        if result == 0 {
            Err(Error::last_os_error())
        } else {
            Ok((creation, exit, kernel, user))
        }
    }
}

impl ProcessHandle<Termination> {
    pub fn for_termination(pid: Pid) -> Result<ProcessHandle<Termination>> {
        let handle = unsafe { processthreadsapi::OpenProcess(winnt::PROCESS_TERMINATE, 0, pid) };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ProcessHandle {
                handle,
                _type: PhantomData,
            })
        }
    }

    /// ERROR_INVALID_PARAMETER should be considered as a `NoSuchProcess` later
    pub fn terminate(&self) -> Result<()> {
        let result = unsafe {
            processthreadsapi::TerminateProcess(
                self.handle,
                // This is going to be the code with which the process will exit
                libc::SIGTERM as u32,
            )
        };
        // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess

        if result == 0 {
            let e = Error::last_os_error();
            // TODO: `psutil` ignores permission errors and considers the target process
            // to be dead already.
            // See: https://github.com/giampaolo/psutil/issues/1099
            // It seems kinda shady, so the behavior should be checked.
            if e.raw_os_error() == Some(winerror::ERROR_ACCESS_DENIED as i32) {
                Ok(())
            } else {
                Err(e)
            }
        } else {
            Ok(())
        }
    }
}

impl ProcessHandle<SuspendResume> {
    pub fn for_suspend_resume(pid: Pid) -> Result<ProcessHandle<SuspendResume>> {
        let handle =
            unsafe { processthreadsapi::OpenProcess(winnt::PROCESS_SUSPEND_RESUME, 0, pid) };

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(ProcessHandle {
                handle,
                _type: PhantomData,
            })
        }
    }

    pub fn suspend(&self) -> Result<()> {
        let result = unsafe { ntpsapi::NtSuspendProcess(self.handle) };

        if result != ntstatus::STATUS_SUCCESS {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn resume(&self) -> Result<()> {
        let result = unsafe { ntpsapi::NtResumeProcess(self.handle) };

        if result != ntstatus::STATUS_SUCCESS {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl<T> Drop for ProcessHandle<T> {
    fn drop(&mut self) {
        let result = unsafe { handleapi::CloseHandle(self.handle) };

        debug_assert!(result != 0);
    }
}
