#![allow(non_camel_case_types)]

use std::ptr;
use std::slice;

use mach::kern_return::{self, kern_return_t};
use mach::mach_types::host_t;
use mach::message::mach_msg_type_number_t;
use mach::traps::mach_task_self;
use mach::vm_types::{integer_t, natural_t, vm_address_t, vm_map_t, vm_size_t};

use heim_common::prelude::*;
use heim_common::sys::macos::host_port;

const PROCESSOR_CPU_LOAD_INFO: libc::c_int = 2;
const CPU_STATE_USER: usize = 0;
const CPU_STATE_SYSTEM: usize = 1;
const CPU_STATE_IDLE: usize = 2;
const CPU_STATE_NICE: usize = 3;

type processor_flavor_t = libc::c_int;
type processor_info_array_t = *mut integer_t;

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
        out_processor_infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    fn vm_deallocate(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
    ) -> kern_return_t;
}

#[allow(trivial_casts)]
pub fn processor_load_info() -> Result<Vec<processor_cpu_load_info>> {
    let port = host_port::HostPort::get();

    let mut cpu_count = 0;
    let mut processor_info: processor_info_array_t = ptr::null_mut();
    let mut cpu_info_count = 0;

    let result = unsafe {
        host_processor_info(
            *port,
            PROCESSOR_CPU_LOAD_INFO,
            &mut cpu_count,
            &mut processor_info,
            &mut cpu_info_count,
        )
    };

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error().with_ffi("host_processor_info"))
    } else {
        let cpu_info = unsafe { slice::from_raw_parts(processor_info, cpu_info_count as usize) };
        // Could use a `::std::mem::transmute` probably, but this is okay too
        let mut stats = Vec::with_capacity(cpu_count as usize);
        for chunk in cpu_info.chunks(4) {
            stats.push(processor_cpu_load_info {
                user: chunk[CPU_STATE_USER] as natural_t,
                system: chunk[CPU_STATE_SYSTEM] as natural_t,
                idle: chunk[CPU_STATE_IDLE] as natural_t,
                nice: chunk[CPU_STATE_NICE] as natural_t,
            })
        }

        let result = unsafe {
            vm_deallocate(
                mach_task_self(),
                processor_info as vm_address_t,
                cpu_info_count as vm_size_t * std::mem::size_of::<natural_t>(),
            )
        };
        if result != kern_return::KERN_SUCCESS {
            return Err(Error::last_os_error().with_ffi("vm_deallocate"));
        }

        Ok(stats)
    }
}
