#![allow(non_camel_case_types)]

// `mach` crate does not provide the following bindings.
// It would be nice to contribute them later.

use std::mem;
use std::ptr;

use mach::mach_types::{host_t, host_name_port_t};
use mach::vm_types::{natural_t, integer_t};
use mach::kern_return::{self, kern_return_t};
use mach::message::mach_msg_type_number_t;
use mach::traps::mach_task_self;
use mach::mach_port;

use heim_common::prelude::*;

pub const HOST_VM_INFO64: libc::c_int = 4;
pub const HOST_VM_INFO64_COUNT: libc::c_uint = 38;

const CTL_HW: libc::c_int = 6;
const CTL_VM: libc::c_int = 2;
const HW_MEMSIZE: libc::c_int = 24;
const VM_SWAPUSAGE: libc::c_int = 5;

/// https://developer.apple.com/documentation/kernel/host_flavor_t?language=objc
type host_flavor_t = integer_t;

/// https://developer.apple.com/documentation/kernel/host_info64_t?language=objc
type host_info64_t = *mut integer_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: libc::uint64_t,
    pub reactivations: libc::uint64_t,
    pub pageins: libc::uint64_t,
    pub pageouts: libc::uint64_t,
    pub faults: libc::uint64_t,
    pub cow_faults: libc::uint64_t,
    pub lookups: libc::uint64_t,
    pub hits: libc::uint64_t,
    pub purges: libc::uint64_t,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: libc::uint64_t,
    pub compressions: libc::uint64_t,
    pub swapins: libc::uint64_t,
    pub swapouts: libc::uint64_t,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: libc::uint64_t,
}


extern "C" {
    fn mach_host_self() -> host_name_port_t;

    /// https://developer.apple.com/documentation/kernel/1502863-host_statistics64?language=objc
    fn host_statistics64(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info_out: host_info64_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;
}

pub unsafe fn host_vm_info() -> Result<vm_statistics64> {
    let port = mach_host_self();
    let mut stats = vm_statistics64::default();
    let count = HOST_VM_INFO64_COUNT;

    let result = host_statistics64(
        port,
        HOST_VM_INFO64,
        &mut stats as *mut _ as host_info64_t,
        // We can't pass the reference to const here,
        // it leads to `EXC_BAD_ACCESS` for some reasons,
        // so we are copying it to a stack and passing a reference to a local copy
        &count,
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

pub unsafe fn hw_memsize() -> Result<usize> {
    let mut name: [i32; 2] = [CTL_HW, HW_MEMSIZE];
    let mut value = 0u64;
    let mut length = mem::size_of::<u64>();

    let result = libc::sysctl(
        name.as_mut_ptr(),
        2,
        &mut value as *mut u64 as *mut libc::c_void,
        &mut length as *mut libc::size_t,
        ptr::null_mut(),
        0,
    );

    if result == 0 {
        Ok(value as usize)
    } else {
        Err(Error::last_os_error())
    }
}

pub unsafe fn vm_swapusage() -> Result<libc::xsw_usage> {
    let mut name: [i32; 2] = [CTL_VM, VM_SWAPUSAGE];
    let mut value: libc::xsw_usage = mem::uninitialized();
    let mut length = mem::size_of::<libc::xsw_usage>();

    let result = libc::sysctl(
        name.as_mut_ptr(),
        2,
        &mut value as *mut _ as *mut libc::c_void,
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
