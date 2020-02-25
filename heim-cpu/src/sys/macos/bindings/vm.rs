#![allow(non_camel_case_types)]

use std::mem;

use mach::kern_return;
use mach::message::mach_msg_type_number_t;
use mach::vm_types::integer_t;

use heim_common::prelude::*;
use heim_common::sys::macos::{self, host_port};

const HOST_VM_INFO: libc::c_int = 2;

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

#[allow(trivial_casts)]
pub fn vm_meter() -> Result<vmmeter> {
    let port = host_port::HostPort::get();
    let mut stats = vmmeter::default();
    let count = mem::size_of::<vmmeter>() / mem::size_of::<integer_t>();

    let result = unsafe {
        macos::host_statistics(
            *port,
            HOST_VM_INFO,
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
