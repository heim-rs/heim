#![allow(non_camel_case_types)]

use mach::kern_return::kern_return_t;
use mach::mach_types::host_t;
use mach::message::mach_msg_type_number_t;
use mach::vm_types::{integer_t, natural_t, vm_address_t, vm_map_t, vm_size_t};

pub const PROCESSOR_CPU_LOAD_INFO: libc::c_int = 2;
pub const HOST_CPU_LOAD_INFO: libc::c_int = 3;
pub const HOST_VM_INFO: libc::c_int = 2;
pub const CPU_STATE_USER: usize = 0;
pub const CPU_STATE_SYSTEM: usize = 1;
pub const CPU_STATE_IDLE: usize = 2;
pub const CPU_STATE_NICE: usize = 3;

pub type processor_flavor_t = libc::c_int;
pub type processor_info_array_t = *mut integer_t;

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
    pub fn host_processor_info(
        host: host_t,
        flavor: processor_flavor_t,
        out_processor_count: *mut natural_t,
        out_processor_info: *mut processor_info_array_t,
        out_processor_infoCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn vm_deallocate(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
    ) -> kern_return_t;
}
