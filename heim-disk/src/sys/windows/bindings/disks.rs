use std::io;
use std::ptr;
use std::mem;
use std::path::Path;
use std::os::windows::io::RawHandle;
use std::os::windows::ffi::OsStrExt;

use winapi::um::{winbase, winnt, winioctl, fileapi, ioapiset};
use winapi::shared::{minwindef, winerror};

use heim_common::prelude::*;

const ERROR_INVALID_FUNCTION: i32 = winerror::ERROR_INVALID_FUNCTION as i32;
const ERROR_NOT_SUPPORTED: i32 = winerror::ERROR_NOT_SUPPORTED as i32;

// Is not declared in the `winapi`
// TODO: Get rid of it when the winapi-rs PR will be merged
// https://github.com/retep998/winapi-rs/pull/765
#[repr(C)]
#[derive(Default)]
#[allow(non_snake_case)]
pub struct DISK_PERFORMANCE {
    pub BytesRead: winnt::LARGE_INTEGER,
    pub BytesWritten: winnt::LARGE_INTEGER,
    pub ReadTime: winnt::LARGE_INTEGER,
    pub WriteTime: winnt::LARGE_INTEGER,
    pub IdleTime: winnt::LARGE_INTEGER,
    pub ReadCount: minwindef::DWORD,
    pub WriteCount: minwindef::DWORD,
    pub QueueDepth: minwindef::DWORD,
    pub SplitCount: minwindef::DWORD,
    pub QueryTime: winnt::LARGE_INTEGER,
    pub StorageDeviceNumber: minwindef::DWORD,
    pub StorageManagerName: [winnt::WCHAR; 8],
}

/// ## Returns
///
/// `DeviceIoControl` might fail in some rare and hardly reproducible conditions.
/// Few of the errors will be ignored (same as psutil does), in that case `Ok(None)`
/// will be returned. Higher level code should ignore such an entries.
/// For reference: https://github.com/giampaolo/psutil/blob/5a398984d709d750da1fc0e450d72c771e18f393/psutil/_psutil_windows.c#L2262-L2277
#[allow(trivial_casts)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub unsafe fn disk_performance(handle: &RawHandle) -> Result<Option<DISK_PERFORMANCE>> {
    let mut perf = DISK_PERFORMANCE::default();
    let mut bytes_returned: minwindef::DWORD = 0;

    let result = ioapiset::DeviceIoControl(
        *handle,
        winioctl::IOCTL_DISK_PERFORMANCE,
        ptr::null_mut(),
        0,
        &mut perf as *mut _ as minwindef::LPVOID,
        mem::size_of::<DISK_PERFORMANCE>() as minwindef::DWORD,
        &mut bytes_returned,
        ptr::null_mut()
    );

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

pub fn is_fixed_drive(path: &Path) -> bool {
    let buffer: Vec<winnt::WCHAR> = path.as_os_str().encode_wide().collect();

    let result = unsafe {
        fileapi::GetDriveTypeW(buffer.as_ptr())
    };

    match result {
        winbase::DRIVE_FIXED => true,
        _ => false,
    }
}
