use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;

use winapi::shared::{minwindef, ntstatus};
use winapi::um::{sysinfoapi, winbase, winnt};

use heim_common::prelude::{Error, Result};

use super::super::bindings::MAX_COMPUTERNAME_LENGTH;

// Partial copy of the `sysinfoapi::SYSTEM_INFO`,
// because it contains pointers and we need to sent it between threads.
// TODO: It would be better to make `SYSTEM_INFO` Sendable somehow?
#[derive(Debug)]
pub struct SystemInfo {
    pub processor_arch: minwindef::WORD,
}

impl From<sysinfoapi::SYSTEM_INFO> for SystemInfo {
    fn from(info: sysinfoapi::SYSTEM_INFO) -> SystemInfo {
        let s = unsafe { info.u.s() };

        SystemInfo {
            processor_arch: s.wProcessorArchitecture,
        }
    }
}

pub fn get_native_system_info() -> SystemInfo {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();

    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo
        sysinfoapi::GetNativeSystemInfo(info.as_mut_ptr());

        info.assume_init().into()
    }
}

pub fn get_computer_name() -> Result<String> {
    let mut buffer: Vec<winnt::WCHAR> = Vec::with_capacity((MAX_COMPUTERNAME_LENGTH + 1) as usize);
    let mut size: minwindef::DWORD = MAX_COMPUTERNAME_LENGTH + 1;

    let result = unsafe { winbase::GetComputerNameW(buffer.as_mut_ptr(), &mut size) };
    if result == 0 {
        Err(Error::last_os_error().with_ffi("GetComputerName"))
    } else {
        unsafe {
            buffer.set_len(size as usize + 1);
        }
        let str = OsString::from_wide(&buffer[..(size as usize)])
            .to_string_lossy()
            .to_string();

        Ok(str)
    }
}

pub fn rtl_get_version() -> Result<winnt::OSVERSIONINFOEXW> {
    let mut osinfo = mem::MaybeUninit::<winnt::RTL_OSVERSIONINFOEXW>::uninit();
    let result = unsafe {
        (*osinfo.as_mut_ptr()).dwOSVersionInfoSize =
            mem::size_of::<winnt::RTL_OSVERSIONINFOEXW>() as minwindef::DWORD;

        ntapi::ntrtl::RtlGetVersion(osinfo.as_mut_ptr() as *mut winnt::RTL_OSVERSIONINFOW)
    };
    if result == ntstatus::STATUS_SUCCESS {
        unsafe { Ok(osinfo.assume_init()) }
    } else {
        Err(Error::last_os_error().with_ffi("RtlGetVersion"))
    }
}
