use std::mem;
use std::ptr;

use heim_common::prelude::*;
use heim_common::sys::IntoTime;
use heim_common::units::Time;

pub fn boot_time() -> impl Future<Output = Result<Time>> {
    let mut name: [i32; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut size: libc::size_t = mem::size_of::<libc::timeval>();
    let mut info = mem::MaybeUninit::<libc::timeval>::uninit();

    let result = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            2,
            info.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            ptr::null_mut(),
            0,
        )
    };

    if result < 0 {
        return future::err(Error::last_os_error());
    }

    let info = unsafe { info.assume_init() };

    future::ok(info.into_time())
}
