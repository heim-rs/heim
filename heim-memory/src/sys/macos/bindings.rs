#![allow(non_camel_case_types)]

// `mach` crate does not provide the following bindings.
// It would be nice to contribute them later.

use mach::vm_types::natural_t;

pub const HOST_VM_INFO64: libc::c_int = 4;
pub const HOST_VM_INFO64_COUNT: libc::c_uint = 38;

pub const CTL_HW: libc::c_int = 6;
pub const CTL_VM: libc::c_int = 2;
pub const HW_MEMSIZE: libc::c_int = 24;
pub const VM_SWAPUSAGE: libc::c_int = 5;

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
