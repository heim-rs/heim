use std::convert::TryFrom;
use std::io;
use std::mem;
use std::ptr;

use heim_common::prelude::{Error2, Result2};

use super::super::bindings::{self, kinfo_proc};
use crate::{Pid, ProcessError, ProcessResult, Status};

impl TryFrom<libc::c_char> for Status {
    type Error = Error2;

    fn try_from(value: libc::c_char) -> Result2<Status> {
        match value {
            bindings::SIDL => Ok(Status::Idle),
            bindings::SRUN => Ok(Status::Running),
            bindings::SSLEEP => Ok(Status::Sleeping),
            bindings::SSTOP => Ok(Status::Stopped),
            bindings::SZOMB => Ok(Status::Zombie),
            other => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);
                Err(Error2::from(inner)
                    .with_message(format!("Unnknown process p_stat {:?}", other)))
            }
        }
    }
}

pub fn processes() -> Result2<Vec<kinfo_proc>> {
    let mut name: [i32; 3] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ALL];
    let mut size: libc::size_t = 0;
    let mut processes: Vec<kinfo_proc> = vec![];

    loop {
        let result = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                3,
                ptr::null_mut(),
                &mut size,
                ptr::null_mut(),
                0,
            )
        };
        if result < 0 {
            return Err(Error2::last_os_error());
        }

        processes.reserve(size);

        let result = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                3,
                processes.as_mut_ptr() as *mut libc::c_void,
                &mut size,
                ptr::null_mut(),
                0,
            )
        };
        match result {
            libc::ENOMEM => continue,
            code if code < 0 => return Err(Error2::last_os_error()),
            _ => {
                let length = size / mem::size_of::<kinfo_proc>();
                unsafe {
                    processes.set_len(length);
                }
                debug_assert!(!processes.is_empty());

                return Ok(processes);
            }
        }
    }
}

pub fn process(pid: Pid) -> ProcessResult<kinfo_proc> {
    let mut name: [i32; 4] = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_PID, pid];
    let mut size: libc::size_t = mem::size_of::<kinfo_proc>();
    let mut info = mem::MaybeUninit::<kinfo_proc>::uninit();

    let result = unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            4,
            info.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            ptr::null_mut(),
            0,
        )
    };

    if result < 0 {
        return Err(Error2::last_os_error().into());
    }

    // TODO: Re-use the `heim_common::sys::macos::sysctl` routines
    // What to do with that `size == 0` check then?

    // sysctl succeeds but size is zero, happens when process has gone away
    if size == 0 {
        return Err(ProcessError::NoSuchProcess(pid));
    }

    unsafe { Ok(info.assume_init()) }
}
