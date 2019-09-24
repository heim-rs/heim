use std::io;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use winapi::shared::{minwindef, winerror};
use winapi::um::{fileapi, handleapi, ioapiset, winioctl, winnt};

use heim_common::prelude::*;

const ERROR_INVALID_FUNCTION: i32 = winerror::ERROR_INVALID_FUNCTION as i32;
const ERROR_NOT_SUPPORTED: i32 = winerror::ERROR_NOT_SUPPORTED as i32;

/// ## Returns
///
/// `DeviceIoControl` might fail in some rare and hardly reproducible conditions.
/// Few of the errors will be ignored (same as psutil does), in that case `Ok(None)`
/// will be returned. Higher level code should ignore such entries.
/// For reference: https://github.com/giampaolo/psutil/blob/5a398984d709d750da1fc0e450d72c771e18f393/psutil/_psutil_windows.c#L2262-L2277
#[allow(trivial_casts)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn disk_performance(volume_path: &Path) -> Result<Option<winioctl::DISK_PERFORMANCE>> {
    let os_str = volume_path.as_os_str();
    let mut path = os_str.encode_wide().collect::<Vec<_>>();
    // Dropping out trailing backslash and replacing it with a \0
    let _ = path.pop();
    path.push(0x0000);

    // Raw `CreateFileW` is used here instead of `heim-runtime` FS shims
    // because we need the `dwDesiredAccess = 0` parameter,
    // which is impossible to achieve with `std::fs` routines at least
    let handle = unsafe {
        fileapi::CreateFileW(
            path.as_ptr(),
            0,
            winnt::FILE_SHARE_READ | winnt::FILE_SHARE_WRITE,
            ptr::null_mut(),
            fileapi::OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    if handle == handleapi::INVALID_HANDLE_VALUE {
        return Err(Error::last_os_error());
    }

    let mut perf = winioctl::DISK_PERFORMANCE::default();
    let mut bytes_returned: minwindef::DWORD = 0;

    let result = unsafe {
        ioapiset::DeviceIoControl(
            handle,
            winioctl::IOCTL_DISK_PERFORMANCE,
            ptr::null_mut(),
            0,
            &mut perf as *mut _ as minwindef::LPVOID,
            mem::size_of::<winioctl::DISK_PERFORMANCE>() as minwindef::DWORD,
            &mut bytes_returned,
            ptr::null_mut(),
        )
    };

    let handle_result = unsafe { handleapi::CloseHandle(handle) };
    if handle_result == 0 {
        return Err(Error::last_os_error());
    }

    if result == 0 {
        let e = io::Error::last_os_error();
        match e.raw_os_error() {
            // See function doc
            Some(ERROR_INVALID_FUNCTION) => Ok(None),
            Some(ERROR_NOT_SUPPORTED) => Ok(None),
            _ => Err(e.into()),
        }
    } else {
        Ok(Some(perf))
    }
}
