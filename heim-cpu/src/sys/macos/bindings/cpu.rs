#![allow(non_camel_case_types)]

use std::mem;

use mach::kern_return::{self};
use mach::message::mach_msg_type_number_t;
use mach::vm_types::{integer_t, natural_t};

use heim_common::prelude::*;
use heim_common::sys::macos::{self, host_port};

const HOST_CPU_LOAD_INFO: libc::c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_cpu_load_info {
    pub user: natural_t,
    pub system: natural_t,
    pub idle: natural_t,
    pub nice: natural_t,
}

#[allow(trivial_casts)]
pub fn cpu_load_info() -> Result<host_cpu_load_info> {
    let port = host_port::HostPort::get();
    let mut stats = host_cpu_load_info::default();
    let count = mem::size_of::<host_cpu_load_info>() / mem::size_of::<integer_t>();

    let result = unsafe {
        macos::host_statistics64(
            *port,
            HOST_CPU_LOAD_INFO,
            &mut stats as *mut _ as macos::host_info64_t,
            &count as *const _ as *const mach_msg_type_number_t,
        )
    };

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error().with_ffi("host_statistics64"))
    } else {
        Ok(stats)
    }
}
