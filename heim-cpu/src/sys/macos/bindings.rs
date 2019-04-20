#![allow(non_camel_case_types)]

use std::mem;
use std::ptr;
use std::ffi::CStr;

use mach::kern_return::{self, kern_return_t};
use mach::traps::mach_task_self;
use mach::mach_port;
use mach::vm_types::{natural_t, integer_t};
use mach::message::mach_msg_type_number_t;
use mach::mach_types::host_t;

use heim_common::sys::macos;
use heim_common::prelude::*;

const PROCESSOR_CPU_LOAD_INFO: libc::c_int = 2;
const HOST_CPU_LOAD_INFO: libc::c_int = 3;

type processor_flavor_t = libc::c_int;
type processor_info_array_t = *mut integer_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_cpu_load_info {
    pub user: natural_t,
    pub system: natural_t,
    pub idle: natural_t,
    pub nice: natural_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct processor_cpu_load_info {
    pub user: natural_t,
    pub system: natural_t,
    pub idle: natural_t,
    pub nice: natural_t,
}

extern "C" {
    fn host_processor_info(
        host: host_t,
        flavor: processor_flavor_t,
        out_processor_count: *mut natural_t,
        out_processor_info: *mut processor_info_array_t,
        out_processor_infoCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;
}

pub unsafe fn cpu_load_info() -> Result<host_cpu_load_info> {
    let port = macos::mach_host_self();
    let mut stats = host_cpu_load_info::default();
    // TODO: Move to const
    let count = mem::size_of::<host_cpu_load_info>() / mem::size_of::<integer_t>();

    let result = macos::host_statistics64(
        port,
        HOST_CPU_LOAD_INFO,
        &mut stats as *mut _ as macos::host_info64_t,
        &count as *const _ as *const mach_msg_type_number_t,
    );

    let port_result = mach_port::mach_port_deallocate(mach_task_self(), port);
    // Technically it is a programming bug and we are should panic probably,
    // but it is okay as is
    if port_result != kern_return::KERN_SUCCESS {
        return Err(Error::last_os_error());
    }

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error())
    } else {
        Ok(stats)
    }
}

pub unsafe fn processor_load_info() -> Result<Vec<processor_cpu_load_info>> {
    let port = macos::mach_host_self();
    let mut stats: Vec<processor_cpu_load_info> = Vec::with_capacity(1);
    // TODO: Move to const
    let count = mem::size_of::<processor_cpu_load_info>() / mem::size_of::<integer_t>();
    let mut result_count: natural_t = 0;

    let result = host_processor_info(
        port,
        PROCESSOR_CPU_LOAD_INFO,
        &mut result_count as *mut _ as *mut natural_t,
        &mut stats.as_mut_ptr() as *mut _  as *mut processor_info_array_t,
        &count as *const _ as *const mach_msg_type_number_t
    );

    let port_result = mach_port::mach_port_deallocate(mach_task_self(), port);
    // Technically it is a programming bug and we are should panic probably,
    // but it is okay as is
    if port_result != kern_return::KERN_SUCCESS {
        return Err(Error::last_os_error());
    }

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error())
    } else {
        stats.set_len(result_count as usize);
        Ok(stats)
    }
}

unsafe fn frequency(key: &[u8]) -> Result<u64> {
    let str = CStr::from_bytes_with_nul_unchecked(key);
    let mut value = 0u64;
    let mut length = mem::size_of::<u64>();

    let result = libc::sysctlbyname(
        str.as_ptr(),
        &mut value as *mut u64 as *mut libc::c_void,
        &mut length as *mut libc::size_t,
        ptr::null_mut(),
        0,
    );

    if result == 0 {
        Ok(value)
    } else {
        Err(Error::last_os_error())
    }
}

// Returns hertz
pub fn cpu_frequency() -> Result<u64> {
    unsafe {
        frequency(b"hw.cpufrequency\0")
    }
}

// Returns hertz
pub fn cpu_frequency_max() -> Result<u64> {
    unsafe {
        frequency(b"hw.cpufrequency_max\0")
    }
}

// Returns hertz
pub fn cpu_frequency_min() -> Result<u64> {
    unsafe {
        frequency(b"hw.cpufrequency_min\0")
    }
}
