#![allow(non_camel_case_types)]

use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::slice;

use mach::kern_return::{self, kern_return_t};
use mach::mach_port;
use mach::mach_types::host_t;
use mach::message::mach_msg_type_number_t;
use mach::traps::mach_task_self;
use mach::vm_types::{integer_t, natural_t, vm_address_t, vm_map_t, vm_size_t};

use heim_common::prelude::*;
use heim_common::sys::macos;

const PROCESSOR_CPU_LOAD_INFO: libc::c_int = 2;
const HOST_CPU_LOAD_INFO: libc::c_int = 3;
const HOST_VM_INFO: libc::c_int = 2;
const CPU_STATE_USER: usize = 0;
const CPU_STATE_SYSTEM: usize = 1;
const CPU_STATE_IDLE: usize = 2;
const CPU_STATE_NICE: usize = 3;

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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vmmeter {
    pub v_swtch: libc::c_uint,
    pub v_trap: libc::c_uint,
    pub v_syscall: libc::c_uint,
    pub v_intr: libc::c_uint,
    pub v_soft: libc::c_uint,
    pub v_faults: libc::c_uint,

    pub v_lookups: libc::c_uint,
    pub v_hits: libc::c_uint,
    pub v_vm_faults: libc::c_uint,
    pub v_cow_faults: libc::c_uint,
    pub v_swpin: libc::c_uint,
    pub v_swpout: libc::c_uint,
    pub v_pswpin: libc::c_uint,
    pub v_pswpout: libc::c_uint,
    pub v_pageins: libc::c_uint,
    pub v_pageouts: libc::c_uint,
    pub v_pgpgin: libc::c_uint,
    pub v_pgpgout: libc::c_uint,
    pub v_intrans: libc::c_uint,
    pub v_reactivated: libc::c_uint,
    pub v_rev: libc::c_uint,
    pub v_scan: libc::c_uint,
    pub v_dfree: libc::c_uint,
    pub v_pfree: libc::c_uint,
    pub v_zfod: libc::c_uint,
    pub v_nzfod: libc::c_uint,

    pub v_page_size: libc::c_uint,
    pub v_kernel_pages: libc::c_uint,
    pub v_free_target: libc::c_uint,
    pub v_free_min: libc::c_uint,
    pub v_free_count: libc::c_uint,
    pub v_wire_count: libc::c_uint,
    pub v_active_count: libc::c_uint,
    pub v_inactive_target: libc::c_uint,
    pub v_inactive_count: libc::c_uint,
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

#[allow(trivial_casts)]
pub unsafe fn processor_load_info() -> Result<Vec<processor_cpu_load_info>> {
    let port = macos::mach_host_self();
    let mut cpu_count = 0;
    let mut processor_info: processor_info_array_t = ptr::null_mut();
    let mut cpu_info_count = 0;

    let result = host_processor_info(
        port,
        PROCESSOR_CPU_LOAD_INFO,
        &mut cpu_count,
        &mut processor_info,
        &mut cpu_info_count,
    );

    let port_result = mach_port::mach_port_deallocate(mach_task_self(), port);
    if port_result != kern_return::KERN_SUCCESS {
        return Err(Error::last_os_error());
    }

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error())
    } else {
        let cpu_info = slice::from_raw_parts(processor_info, cpu_info_count as usize);
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

        let result = vm_deallocate(
            mach_task_self(),
            processor_info as vm_address_t,
            cpu_info_count as vm_size_t * std::mem::size_of::<natural_t>(),
        );
        if result != kern_return::KERN_SUCCESS {
            return Err(Error::last_os_error());
        }

        Ok(stats)
    }
}

#[allow(trivial_casts)]
pub unsafe fn vm_meter() -> Result<vmmeter> {
    let port = macos::mach_host_self();
    let mut stats = vmmeter::default();
    // TODO: Move to const
    let count = mem::size_of::<vmmeter>() / mem::size_of::<integer_t>();

    let result = macos::host_statistics(
        port,
        HOST_VM_INFO,
        &mut stats as *mut _ as macos::host_info_t,
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

#[allow(trivial_casts)]
unsafe fn frequency(key: &[u8]) -> Result<u64> {
    let str = CStr::from_bytes_with_nul_unchecked(key);
    let mut value = 0u64;
    let mut length = mem::size_of::<u64>();

    let result = libc::sysctlbyname(
        str.as_ptr(),
        &mut value as *mut _ as *mut libc::c_void,
        &mut length,
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
    unsafe { frequency(b"hw.cpufrequency\0") }
}

// Returns hertz
pub fn cpu_frequency_max() -> Result<u64> {
    unsafe { frequency(b"hw.cpufrequency_max\0") }
}

// Returns hertz
pub fn cpu_frequency_min() -> Result<u64> {
    unsafe { frequency(b"hw.cpufrequency_min\0") }
}
