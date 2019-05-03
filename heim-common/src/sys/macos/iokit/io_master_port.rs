use core_foundation::base::{mach_port_t, kCFNull};
use mach::{port, mach_port, kern_return, traps};

use crate::{Result, Error};
use super::{ffi, IoIterator};

#[derive(Debug)]
pub struct IoMasterPort(mach_port_t);

impl IoMasterPort {
    pub fn new() -> Result<IoMasterPort> {
        let mut master_port: port::mach_port_t = port::MACH_PORT_NULL;

        let result = unsafe {
            ffi::IOMasterPort(ffi::kIOMasterPortDefault, &mut master_port)
        };

        if result != kern_return::KERN_SUCCESS {
            Err(Error::last_os_error())
        } else {
            Ok(IoMasterPort(master_port))
        }
    }

    /// Returns services matching the `name`,
    /// where `name` is bytes string ending with a `0x00`,
    /// for example: `b"IOMedia\0"`
    pub fn get_services(&self, name: &[u8]) -> Result<IoIterator> {
        let service = unsafe {
            let ret = ffi::IOServiceMatching(name.as_ptr() as *const libc::c_char);
            assert_ne!(ret as *const _, kCFNull);

            ret
        };

        let mut iterator = IoIterator::default();

        let result = unsafe {
            ffi::IOServiceGetMatchingServices(self.0, service, &mut *iterator)
        };

        if result == kern_return::KERN_SUCCESS {
            Ok(iterator)
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Drop for IoMasterPort {
    fn drop(&mut self) {
        let result = unsafe {
            mach_port::mach_port_deallocate(traps::mach_task_self(), self.0)
        };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}
