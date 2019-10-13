use std::mem;
use std::ptr;
use std::slice;

use mach::kern_return::{self};
use mach::message::mach_msg_type_number_t;
use mach::traps::mach_task_self;
use mach::vm_types::{integer_t, natural_t, vm_address_t, vm_size_t};

use heim_common::prelude::{Error, Result};
use heim_common::sys::macos::{self, host_port::HostPort, sysctl};

use super::bindings::{self, host_cpu_load_info, host_processor_info};

#[allow(trivial_casts)]
pub fn cpu_load_info() -> Result<host_cpu_load_info> {
    let port = HostPort::get();
    let mut stats = host_cpu_load_info::default();
    // TODO: Move to const
    let count = mem::size_of::<host_cpu_load_info>() / mem::size_of::<integer_t>();

    let result = unsafe {
        macos::host_statistics64(
            port.to_inner(),
            bindings::HOST_CPU_LOAD_INFO,
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

#[allow(trivial_casts)]
pub fn processor_load_info() -> Result<Vec<bindings::processor_cpu_load_info>> {
    let port = HostPort::get();
    let mut cpu_count = 0;
    let mut processor_info: bindings::processor_info_array_t = ptr::null_mut();
    let mut cpu_info_count = 0;

    let result = unsafe {
        host_processor_info(
            port.to_inner(),
            bindings::PROCESSOR_CPU_LOAD_INFO,
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
            stats.push(bindings::processor_cpu_load_info {
                user: chunk[bindings::CPU_STATE_USER] as natural_t,
                system: chunk[bindings::CPU_STATE_SYSTEM] as natural_t,
                idle: chunk[bindings::CPU_STATE_IDLE] as natural_t,
                nice: chunk[bindings::CPU_STATE_NICE] as natural_t,
            })
        }

        let result = unsafe {
            bindings::vm_deallocate(
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

#[allow(trivial_casts)]
pub fn vm_meter() -> Result<bindings::vmmeter> {
    let port = HostPort::get();
    let mut stats = bindings::vmmeter::default();
    // TODO: Move to const
    let count = mem::size_of::<bindings::vmmeter>() / mem::size_of::<integer_t>();

    let result = unsafe {
        macos::host_statistics(
            port.to_inner(),
            bindings::HOST_VM_INFO,
            &mut stats as *mut _ as macos::host_info_t,
            &count as *const _ as *const mach_msg_type_number_t,
        )
    };

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error().with_ffi("host_statistics"))
    } else {
        Ok(stats)
    }
}

// Returns hertz
pub fn cpu_frequency() -> Result<u64> {
    unsafe { sysctl::sysctlbyname(b"hw.cpufrequency\0") }
        .map_err(|e| Error::from(e).with_named_syscall("hw.cpufrequency"))
}

// Returns hertz
pub fn cpu_frequency_max() -> Result<u64> {
    unsafe { sysctl::sysctlbyname(b"hw.cpufrequency_max\0") }
        .map_err(|e| Error::from(e).with_named_syscall("hw.cpufrequency_max"))
}

// Returns hertz
pub fn cpu_frequency_min() -> Result<u64> {
    unsafe { sysctl::sysctlbyname(b"hw.cpufrequency_min\0") }
        .map_err(|e| Error::from(e).with_named_syscall("hw.cpufrequency_min"))
}
