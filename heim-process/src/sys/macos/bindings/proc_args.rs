use std::io;
use std::mem;
use std::ptr;

use heim_common::sys::macos::sysctl;
use heim_common::{Error, Result};

use crate::Pid;

pub fn args_max() -> Result<libc::c_int> {
    sysctl::sysctl(&mut [libc::CTL_KERN, libc::KERN_ARGMAX])
}

// TODO: https://chromium.googlesource.com/crashpad/crashpad/+/360e441c53ab4191a6fd2472cc57c3343a2f6944/util/posix/process_util_mac.cc#32
// TODO: Use `process::ProcessResult`
#[allow(trivial_casts)]
pub fn proc_args(pid: Pid) -> Result<Vec<u8>> {
    // Command line for `kernel_task` process can't be fetched
    if pid == 0 {
        // TODO: Return `ProcessError::AccessDenied`
        return Err(io::Error::from(io::ErrorKind::PermissionDenied).into());
    }

    let mut args_max = args_max()? as usize;
    let mut name: [libc::c_int; 3] = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
    let mut value = Vec::with_capacity(args_max);

    let result = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            3,
            value.as_mut_ptr() as *mut libc::c_void,
            &mut args_max,
            ptr::null_mut(),
            0,
        )
    };

    if args_max < mem::size_of::<libc::c_int>() {
        let inner = io::Error::from(io::ErrorKind::InvalidData);
        // TODO: context message
        return Err(Error::from(inner));
    }

    // `sysctl` changes the `args_max` value to what length were stored
    // and this is very convenient for us.
    unsafe {
        value.set_len(args_max);
    }

    // Since the default `argmax` is a rather big value (~256kb)
    // it would be unreasonable to keep all that unused data later
    // while the successful result is alive
    value.shrink_to_fit();

    if result == 0 {
        Ok(value)
    } else {
        Err(Error::last_os_error().with_sysctl(name.as_ref()))
    }
}
