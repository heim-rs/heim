#![allow(non_camel_case_types)]

// `mach` crate does not provide the following bindings.
// It would be nice to contribute them later.

use mach::kern_return;
use mach::vm_types::natural_t;

use heim_common::prelude::{Error, Result};
use heim_common::sys::macos::{self, host_port, sysctl};

pub const HOST_VM_INFO64: libc::c_int = 4;
pub const HOST_VM_INFO64_COUNT: libc::c_uint = 38;

const CTL_HW: libc::c_int = 6;
const CTL_VM: libc::c_int = 2;
const HW_MEMSIZE: libc::c_int = 24;
const VM_SWAPUSAGE: libc::c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}

#[allow(trivial_casts)]
pub fn host_vm_info() -> Result<vm_statistics64> {
    let port = host_port::HostPort::get();
    let mut stats = vm_statistics64::default();
    let count = HOST_VM_INFO64_COUNT;

    let result = unsafe {
        macos::host_statistics64(
            *port,
            HOST_VM_INFO64,
            &mut stats as *mut _ as macos::host_info64_t,
            // We can't pass the reference to const here,
            // it leads to `EXC_BAD_ACCESS` for some reasons,
            // so we are copying it to a stack and passing a reference to a local copy
            &count,
        )
    };

    if result != kern_return::KERN_SUCCESS {
        Err(Error::last_os_error().with_ffi("host_statistics64"))
    } else {
        Ok(stats)
    }
}

pub fn hw_memsize() -> Result<u64> {
    sysctl::sysctl(&mut [CTL_HW, HW_MEMSIZE])
}

pub fn vm_swapusage() -> Result<libc::xsw_usage> {
    sysctl::sysctl(&mut [CTL_VM, VM_SWAPUSAGE])
}
