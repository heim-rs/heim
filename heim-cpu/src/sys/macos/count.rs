use std::mem;
use std::ptr;
use std::ffi::CStr;

use heim_common::prelude::*;


pub fn logical_count() -> impl Future<Output = Result<u64>> {
    let str = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"hw.logicalcpu\0")
    };
    let mut value = 0i32;
    let mut length = mem::size_of::<i32>();

    let result = unsafe {
        libc::sysctlbyname(
            str.as_ptr(),
            &mut value as *mut i32 as *mut libc::c_void,
            &mut length as *mut libc::size_t,
            ptr::null_mut(),
            0,
        )
    };

    if result == 0 {
        future::ok(value as u64)
    } else {
        future::err(Error::last_os_error())
    }
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    // TODO: Stub, see https://github.com/heim-rs/heim/issues/53
    future::ok(None)
}
