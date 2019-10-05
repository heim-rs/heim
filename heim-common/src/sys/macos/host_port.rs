use std::io;

use mach::{kern_return, mach_port, mach_types::host_name_port_t, traps::mach_task_self};

use super::mach_host_self;

#[derive(Debug)]
pub struct HostPort(host_name_port_t);

impl HostPort {
    pub fn get() -> HostPort {
        let inner = unsafe { mach_host_self() };

        HostPort(inner)
    }

    pub fn to_inner(&self) -> host_name_port_t {
        self.0
    }

    pub fn deallocate(&mut self) -> io::Result<()> {
        let result = unsafe { mach_port::mach_port_deallocate(mach_task_self(), self.0) };

        if result == kern_return::KERN_SUCCESS {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

impl Drop for HostPort {
    fn drop(&mut self) {
        self.deallocate().expect("Unable to close mach port");
    }
}
