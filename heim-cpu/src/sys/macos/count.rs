use std::ffi::CStr;
use std::mem;
use std::ptr;

use heim_common::prelude::*;

#[allow(trivial_casts)]
fn sysctl(key: &[u8]) -> Result<u64> {
    let str = unsafe { CStr::from_bytes_with_nul_unchecked(key) };
    let mut value = 0i32;
    let mut length = mem::size_of::<i32>();

    let result = unsafe {
        libc::sysctlbyname(
            str.as_ptr(),
            &mut value as *mut i32 as *mut libc::c_void,
            &mut length,
            ptr::null_mut(),
            0,
        )
    };

    if result == 0 {
        Ok(value as u64)
    } else {
        Err(Error::last_os_error())
    }
}

pub fn logical_count() -> impl Future<Output = Result<u64>> {
    future::ready(sysctl(b"hw.logicalcpu\0"))
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    future::ready(sysctl(b"hw.physicalcpu\0").map(Some))
}
