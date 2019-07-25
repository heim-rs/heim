use std::path::PathBuf;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;

use heim_common::prelude::*;

use crate::{Pid, ProcessResult, ProcessError};

extern "C" {
    pub fn proc_pidpath(pid: libc::c_int, buffer: *mut libc::c_void, buffersize: u32) -> libc::c_int;
}

pub fn pid_path(pid: Pid) -> ProcessResult<PathBuf> {
    let mut buffer: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let result = unsafe {
        proc_pidpath(pid, buffer.as_mut_ptr() as *mut libc::c_void, libc::PATH_MAX as u32)
    };

    if result == 0 {
        if pid == 0 {
            Err(ProcessError::AccessDenied(pid))
        } else {
            Err(Error::last_os_error().into())
        }
    } else {
        let c_str = unsafe {
            CStr::from_ptr(buffer.as_ptr())
        };
        let os_str = OsStr::from_bytes(c_str.to_bytes());

        // TODO: `to_os_string` introduces another allocation, would be nice to re-use `buffer`
        Ok(os_str.to_os_string().into())
    }
}
