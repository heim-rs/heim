/// https://docs.microsoft.com/en-US/windows/desktop/api/sysinfoapi/ns-sysinfoapi-_memorystatusex

use std::io;
use std::mem;

use winapi::shared::minwindef;
use winapi::um::sysinfoapi;

use heim_common::prelude::*;
use heim_common::units::iec::information::byte;
use heim_common::units::iec::usize::Information;

#[derive(Clone)]
pub struct Memory(sysinfoapi::MEMORYSTATUSEX);

impl Memory {
    pub fn total(&self) -> Information {
        Information::new::<byte>(self.0.ullTotalPhys as usize)
    }

    pub fn available(&self) -> Information {
        Information::new::<byte>(self.0.ullAvailPhys as usize)
    }

    pub fn free(&self) -> Information {
        self.available()
    }
}

#[derive(Clone)]
pub struct Swap(sysinfoapi::MEMORYSTATUSEX);

impl Swap {
    pub fn total(&self) -> Information {
        Information::new::<byte>(self.0.ullTotalPageFile as usize)
    }

    pub fn used(&self) -> Information {
        self.total() - self.free()
    }

    pub fn free(&self) -> Information {
        Information::new::<byte>(self.0.ullAvailPageFile as usize)
    }

}

fn memory_status() -> impl Future<Item=sysinfoapi::MEMORYSTATUSEX, Error=Error> {
    future::lazy(|| {
        unsafe {
            let mut mem_status = mem::uninitialized::<sysinfoapi::MEMORYSTATUSEX>();
            mem_status.dwLength = mem::size_of::<sysinfoapi::MEMORYSTATUSEX>() as minwindef::DWORD;

            let result = sysinfoapi::GlobalMemoryStatusEx(&mut mem_status);
            if result == 0 {
                Err(io::Error::last_os_error().into())
            } else {
                Ok(mem_status)
            }
        }
    })
}

pub fn swap() -> impl Future<Item=Swap, Error=Error> {
    memory_status()
        .map(Swap)
}


pub fn memory() -> impl Future<Item=Memory, Error=Error> {
    memory_status()
        .map(Memory)
}
