//! Process handle variant for querying process information
//! without requiring any additional privileges (expected to work for any user)

use std::convert::TryFrom;
use std::ffi::OsString;
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::{DWORD, FILETIME, MAX_PATH};
use winapi::um::{processthreadsapi, psapi, winbase, winnt};

use heim_common::sys::IntoTime;
use heim_common::units::{time, Time};
use heim_common::Error;

use super::{ProcessHandle, ProcessHandlePermissions};
use crate::os::windows::Priority;
use crate::sys::windows::process::CpuTime;
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug)]
pub struct QueryLimitedInformation;
impl ProcessHandlePermissions for QueryLimitedInformation {
    const ACCESS: DWORD = winnt::PROCESS_QUERY_LIMITED_INFORMATION | winnt::PROCESS_VM_READ;
}

impl ProcessHandle<QueryLimitedInformation> {
    pub fn query_limited_info(pid: Pid) -> ProcessResult<ProcessHandle<QueryLimitedInformation>> {
        let handle = QueryLimitedInformation::open(pid)?;

        Ok(ProcessHandle {
            handle,
            pid,
            _type: PhantomData,
        })
    }

    pub fn exit_code(&self) -> ProcessResult<DWORD> {
        let mut code: DWORD = 0;

        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess
            processthreadsapi::GetExitCodeProcess(*self.handle, &mut code)
        };

        if result == 0 {
            let e = Error::last_os_error().with_ffi("GetExitCodeProcess");
            if e.as_inner().kind() == io::ErrorKind::PermissionDenied {
                Err(ProcessError::AccessDenied(self.pid))
            } else {
                Err(e.into())
            }
        } else {
            Ok(code)
        }
    }

    pub fn exe(&self) -> ProcessResult<PathBuf> {
        let mut buffer: [wchar_t; MAX_PATH] = [0; MAX_PATH];
        let mut size = MAX_PATH as DWORD;

        let result = unsafe {
            winbase::QueryFullProcessImageNameW(*self.handle, 0, buffer.as_mut_ptr(), &mut size)
        };

        if result == 0 {
            Err(Error::last_os_error()
                .with_ffi("QueryFullProcessImageNameW")
                .into())
        } else {
            Ok(OsString::from_wide(&buffer[..(size as usize)]).into())
        }
    }

    pub fn memory(&self) -> ProcessResult<psapi::PROCESS_MEMORY_COUNTERS_EX> {
        let mut counters = mem::MaybeUninit::<psapi::PROCESS_MEMORY_COUNTERS_EX>::uninit();

        let result = unsafe {
            psapi::GetProcessMemoryInfo(
                *self.handle,
                // Tricking the type checker,
                // as the `winapi`' GetProcessMemoryInfo expects `PROCESS_MEMORY_COUNTERS`,
                // not the `PROCESS_MEMORY_COUNTERS_EX`
                counters.as_mut_ptr() as *mut psapi::PROCESS_MEMORY_COUNTERS,
                mem::size_of::<psapi::PROCESS_MEMORY_COUNTERS_EX>() as DWORD,
            )
        };

        if result == 0 {
            Err(Error::last_os_error()
                .with_ffi("GetProcessMemoryInfo")
                .into())
        } else {
            unsafe { Ok(counters.assume_init()) }
        }
    }

    pub fn io_counters(&self) -> ProcessResult<winnt::IO_COUNTERS> {
        let mut counters = mem::MaybeUninit::<winnt::IO_COUNTERS>::uninit();

        let result = unsafe { winbase::GetProcessIoCounters(*self.handle, counters.as_mut_ptr()) };

        if result == 0 {
            Err(Error::last_os_error()
                .with_ffi("GetProcessIoCounters")
                .into())
        } else {
            unsafe { Ok(counters.assume_init()) }
        }
    }

    pub fn cpu_time(&self) -> ProcessResult<CpuTime> {
        let (_, _, kernel, user) = self.process_times()?;

        Ok(CpuTime {
            user: user.into_time(),
            kernel: kernel.into_time(),
        })
    }

    pub fn create_time(&self) -> ProcessResult<Time> {
        let (creation, _, _, _) = self.process_times()?;
        // Seconds amount between the "Windows epoch" (January 1, 1601)
        // and the Unix epoch (January 1, 1970).
        // TODO: It would be nice to make it const,
        // as soon as `uom` will mark `Time::new` as a `const fn`
        let unix_epoch_delta = Time::new::<time::second>(11_644_473_600.0);

        Ok(creation.into_time() - unix_epoch_delta)
    }

    /// Get process priority.
    ///
    /// Note that `set_priority` is located at `ProcessHandle<SetInformation>` impl
    pub fn priority(&self) -> ProcessResult<Priority> {
        let result = unsafe { processthreadsapi::GetPriorityClass(*self.handle) };
        if result == 0 {
            Err(Error::last_os_error().with_ffi("GetPriorityClass").into())
        } else {
            Priority::try_from(result).map_err(Into::into)
        }
    }

    fn process_times(&self) -> ProcessResult<(FILETIME, FILETIME, FILETIME, FILETIME)> {
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        let result = unsafe {
            processthreadsapi::GetProcessTimes(
                *self.handle,
                &mut creation,
                &mut exit,
                &mut kernel,
                &mut user,
            )
        };

        if result == 0 {
            Err(Error::last_os_error().with_ffi("GetProcessTimes").into())
        } else {
            Ok((creation, exit, kernel, user))
        }
    }
}
