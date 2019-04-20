use std::ptr;
use std::mem;
use std::path::Path;
use std::os::windows::io::RawHandle;
use std::os::windows::ffi::OsStrExt;

use winapi::um::{winbase, winnt, winioctl, fileapi, ioapiset};
use winapi::shared::minwindef;

use heim_common::prelude::*;

// Is not declared in the `winapi`
// TODO: Would be nice to contribute it into `winapi` later
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

pub unsafe fn disk_performance(handle: &RawHandle) -> Result<DISK_PERFORMANCE> {
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
        Err(Error::last_os_error())
    } else {
        Ok(perf)
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
