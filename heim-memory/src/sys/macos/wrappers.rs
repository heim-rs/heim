use mach::kern_return;

use heim_common::prelude::{Error, Result};
use heim_common::sys::macos;
use heim_common::sys::macos::{host_port::HostPort, sysctl};

use super::bindings::{
    vm_statistics64, CTL_HW, CTL_VM, HOST_VM_INFO64, HOST_VM_INFO64_COUNT, HW_MEMSIZE, VM_SWAPUSAGE,
};

#[allow(trivial_casts)]
pub fn host_vm_info() -> Result<vm_statistics64> {
    let port = HostPort::get();
    let mut stats = vm_statistics64::default();
    let count = HOST_VM_INFO64_COUNT;

    let result = unsafe {
        macos::host_statistics64(
            port.to_inner(),
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

#[allow(trivial_casts)]
pub fn hw_memsize() -> Result<u64> {
    let mut name: [i32; 2] = [CTL_HW, HW_MEMSIZE];

    sysctl::sysctl(&mut name).map_err(|e| Error::from(e).with_sysctl(&name))
}

pub fn vm_swapusage() -> Result<libc::xsw_usage> {
    let mut name: [i32; 2] = [CTL_VM, VM_SWAPUSAGE];

    sysctl::sysctl(&mut name).map_err(|e| Error::from(e).with_sysctl(&name))
}
