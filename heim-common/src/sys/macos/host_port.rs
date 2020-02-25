//! Safe wrapper around the macOS host ports.

use std::ops;

use mach::{kern_return, mach_port, mach_types::host_name_port_t, traps::mach_task_self};

use super::mach_host_self;
use crate::{Error, Result};

#[derive(Debug)]
pub struct HostPort(host_name_port_t);

impl HostPort {
    /// Returns the task's host self port.
    pub fn get() -> HostPort {
        let inner = unsafe { mach_host_self() };

        HostPort(inner)
    }

    fn deallocate(&mut self) -> Result<()> {
        let result = unsafe { mach_port::mach_port_deallocate(mach_task_self(), self.0) };

        if result == kern_return::KERN_SUCCESS {
            Ok(())
        } else {
            Err(Error::last_os_error().with_ffi("mach_port_deallocate"))
        }
    }
}

impl Drop for HostPort {
    fn drop(&mut self) {
        self.deallocate().expect("Unable to close mach port");
    }
}

impl ops::Deref for HostPort {
    type Target = host_name_port_t;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
