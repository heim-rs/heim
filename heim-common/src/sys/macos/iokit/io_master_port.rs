use std::mem;

use core_foundation::base::{kCFNull, mach_port_t};
use mach::{kern_return, mach_port, port, traps};

use super::{ffi, IoIterator};
use crate::{Error, Result};

/// Safe wrapper around the IOKit master port.
#[derive(Debug)]
pub struct IoMasterPort(mach_port_t);

impl IoMasterPort {
    /// Gets the new IO master port for communication with IOKit.
    pub fn new() -> Result<IoMasterPort> {
        let mut master_port: port::mach_port_t = port::MACH_PORT_NULL;

        let result = unsafe { ffi::IOMasterPort(ffi::kIOMasterPortDefault, &mut master_port) };

        if result != kern_return::KERN_SUCCESS {
            Err(Error::last_os_error())
        } else {
            Ok(IoMasterPort(master_port))
        }
    }

    /// Returns services matching `name`,
    /// where `name` is a bytes string ending in a `0x00`,
    /// for example: `b"IOMedia\0"`
    pub fn get_services(&self, name: &[u8]) -> Result<IoIterator> {
        let service = unsafe {
            let ret = ffi::IOServiceMatching(name.as_ptr() as *const libc::c_char);
            assert_ne!(ret as *const _, kCFNull);

            ret
        };

        let mut raw_iterator = mem::MaybeUninit::<ffi::io_iterator_t>::uninit();

        let result = unsafe {
            ffi::IOServiceGetMatchingServices(self.0, service, raw_iterator.as_mut_ptr())
        };

        if result == kern_return::KERN_SUCCESS {
            let raw_iterator = unsafe { raw_iterator.assume_init() };
            Ok(raw_iterator.into())
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Drop for IoMasterPort {
    fn drop(&mut self) {
        let result = unsafe { mach_port::mach_port_deallocate(traps::mach_task_self(), self.0) };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}
