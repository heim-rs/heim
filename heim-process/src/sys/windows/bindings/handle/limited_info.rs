//! Process handle variant for querying process information
//! without requiring any additional privileges (expected to work for any user)

use std::convert::TryFrom;
use std::ffi::OsString;
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;

use ntapi::{ntpebteb, ntpsapi, ntrtl, ntwow64};
use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::{DWORD, FILETIME, MAX_PATH};
use winapi::shared::{basetsd, ntstatus, winerror};
use winapi::um::{memoryapi, processthreadsapi, psapi, winbase, winnt, wow64apiset};

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

    /// Returns true if the process is a 32-bit x86 process running under 64-bit
    /// x86 Windows.
    fn is_wow64(&self) -> ProcessResult<bool> {
        let mut ret = 0;
        let result = unsafe {
            // This will fail on XP, since it requires PROCESS_QUERY_INFORMATION
            // there.
            wow64apiset::IsWow64Process(*self.handle, &mut ret)
        };

        if result == 0 {
            Err(Error::last_os_error().with_ffi("IsWow64Process").into())
        } else {
            Ok(ret != 0)
        }
    }

    unsafe fn read_memory(&self, src: u64, data: *mut c_void, len: usize) -> ProcessResult<()> {
        // TODO: Use NtWow64ReadVirtualMemory64 when inspecting a 64-bit process
        // from a 32-bit process.

        // Dummy value. Never returned, only set to satisfy compiler.
        let mut err = Error::last_os_error();

        // ReadProcessMemory may fail with ERROR_PARTIAL_COPY, see:
        // https://github.com/giampaolo/psutil/issues/875
        for _i in 0..5 {
            let ret =
                memoryapi::ReadProcessMemory(*self.handle, src as _, data, len, ptr::null_mut());

            if ret == 0 {
                err = Error::last_os_error();
                if err.raw_os_error() != Some(winerror::ERROR_PARTIAL_COPY as _) {
                    return Err(err.with_ffi("ReadProcessMemory").into());
                }
            } else {
                return Ok(());
            }
        }

        Err(err.with_ffi("ReadProcessMemory").into())
    }

    fn get_process_basic_information(&self) -> ProcessResult<ntpsapi::PROCESS_BASIC_INFORMATION> {
        let mut pbi = mem::MaybeUninit::<ntpsapi::PROCESS_BASIC_INFORMATION>::uninit();
        let status = unsafe {
            ntpsapi::NtQueryInformationProcess(
                *self.handle,
                ntpsapi::ProcessBasicInformation,
                pbi.as_mut_ptr() as *mut _,
                mem::size_of::<ntpsapi::PROCESS_BASIC_INFORMATION>() as _,
                ptr::null_mut(),
            )
        };

        if status == ntstatus::STATUS_SUCCESS {
            unsafe { Ok(pbi.assume_init()) }
        } else {
            Err(Error::from_raw_os_error(status | winerror::FACILITY_NT_BIT)
                .with_ffi("NtQueryInformationProcessBasicInfo")
                .into())
        }
    }

    fn get_peb32(&self) -> ProcessResult<Option<ntwow64::PEB32>> {
        let mut peb32_remote_addr: basetsd::ULONG_PTR = 0;

        #[allow(trivial_casts)] // wtf rust.
        let ret = unsafe {
            ntpsapi::NtQueryInformationProcess(
                *self.handle,
                ntpsapi::ProcessWow64Information,
                &mut peb32_remote_addr as *mut _ as _,
                mem::size_of::<basetsd::ULONG_PTR>() as _,
                ptr::null_mut(),
            )
        };

        if ret != ntstatus::STATUS_SUCCESS {
            return Err(Error::from_raw_os_error(ret | winerror::FACILITY_NT_BIT)
                .with_ffi("NtQueryInformationProcessPeb32")
                .into());
        }

        if peb32_remote_addr == 0 {
            return Ok(None);
        }

        let mut peb32 = mem::MaybeUninit::<ntwow64::PEB32>::uninit();

        // read PEB
        unsafe {
            self.read_memory(
                peb32_remote_addr as _,
                peb32.as_mut_ptr() as _,
                mem::size_of::<ntwow64::PEB32>(),
            )?;

            Ok(Some(peb32.assume_init()))
        }
    }

    fn get_process_parameters32(
        &self,
    ) -> ProcessResult<Option<ntwow64::RTL_USER_PROCESS_PARAMETERS32>> {
        let peb32 = self.get_peb32()?;
        let peb32 = if let Some(peb32) = peb32 {
            peb32
        } else {
            return Ok(None);
        };

        let mut params = mem::MaybeUninit::<ntwow64::RTL_USER_PROCESS_PARAMETERS32>::uninit();

        unsafe {
            self.read_memory(
                peb32.ProcessParameters as _,
                params.as_mut_ptr() as _,
                mem::size_of::<ntwow64::RTL_USER_PROCESS_PARAMETERS32>(),
            )?;

            Ok(Some(params.assume_init()))
        }
    }

    // TODO: Define PEB64 to allow 32-bit processes to inspect 64-bit processes
    fn get_peb(&self) -> ProcessResult<Option<ntpebteb::PEB>> {
        if self.is_wow64()? {
            // If remote process is 32-bit (running under WOW64), we won't be
            // able to get its PEB (but we can get its PEB32!).
            return Ok(None);
        }

        // TODO: if *current process*.is_wow64() && !self.is_wow64(), error

        let pbi = self.get_process_basic_information()?;

        let mut peb64 = mem::MaybeUninit::<ntpebteb::PEB>::uninit();

        // read PEB
        unsafe {
            self.read_memory(
                pbi.PebBaseAddress as _,
                peb64.as_mut_ptr() as _,
                mem::size_of::<ntpebteb::PEB>(),
            )?;

            Ok(Some(peb64.assume_init()))
        }
    }

    fn get_process_parameters(&self) -> ProcessResult<Option<ntrtl::RTL_USER_PROCESS_PARAMETERS>> {
        let peb = self.get_peb()?;
        let peb = if let Some(peb) = peb {
            peb
        } else {
            return Ok(None);
        };

        let mut params = mem::MaybeUninit::<ntrtl::RTL_USER_PROCESS_PARAMETERS>::uninit();

        unsafe {
            self.read_memory(
                peb.ProcessParameters as _,
                params.as_mut_ptr() as _,
                mem::size_of::<ntrtl::RTL_USER_PROCESS_PARAMETERS>(),
            )?;

            Ok(Some(params.assume_init()))
        }
    }

    pub fn cwd(&self) -> ProcessResult<PathBuf> {
        let (src, len) = if let Some(params) = self.get_process_parameters32()? {
            let src = params.CurrentDirectory.DosPath.Buffer;
            let len = params.CurrentDirectory.DosPath.Length;
            (src as u64, len)
        } else if let Some(params) = self.get_process_parameters()? {
            let src = params.CurrentDirectory.DosPath.Buffer;
            let len = params.CurrentDirectory.DosPath.Length;
            (src as u64, len)
        } else {
            return Err(ProcessError::UnreadablePeb(self.pid));
        };

        let mut buf = vec![0u16; len as usize / 2];

        unsafe {
            self.read_memory(src, buf.as_mut_ptr() as _, len as _)?;
        };

        Ok(PathBuf::from(OsString::from_wide(&buf)))
    }
}
