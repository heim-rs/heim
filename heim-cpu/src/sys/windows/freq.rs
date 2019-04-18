use std::ptr;
use std::mem;

use winapi::um::{powerbase, sysinfoapi, winnt};
use winapi::shared::{ntstatus, minwindef};

use heim_common::prelude::*;

use crate::units;

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
struct PROCESSOR_POWER_INFORMATION {
    Number: minwindef::ULONG,
    MaxMhz: minwindef::ULONG,
    CurrentMhz: minwindef::ULONG,
    MhzLimit: minwindef::ULONG,
    MaxIdleState: minwindef::ULONG,
    CurrentIdleState: minwindef::ULONG,
}

#[derive(Debug)]
pub struct CpuFrequency(PROCESSOR_POWER_INFORMATION);

impl CpuFrequency {
    pub fn current(&self) -> units::Frequency {
        units::Frequency::new::<units::megahertz>(self.0.CurrentMhz.into())
    }

    pub fn max(&self) -> Option<units::Frequency> {
        Some(units::Frequency::new::<units::megahertz>(self.0.MaxMhz.into()))
    }

    pub fn min(&self) -> Option<units::Frequency> {
        None
    }
}

unsafe fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info: sysinfoapi::SYSTEM_INFO = mem::uninitialized();
    sysinfoapi::GetSystemInfo(&mut info);

    info
}

unsafe fn get_processors() -> Result<Vec<PROCESSOR_POWER_INFORMATION>> {
    let info = get_system_info();
    if info.dwNumberOfProcessors == 0 {
        return Err(Error::new(ErrorKind::UnknownValue))
    }

    let proc_amount = info.dwNumberOfProcessors as usize;
    let mut processors = Vec::<PROCESSOR_POWER_INFORMATION>::with_capacity(proc_amount);
    let buffer_length = proc_amount * mem::size_of::<PROCESSOR_POWER_INFORMATION>();

    let result = powerbase::CallNtPowerInformation(
        winnt::ProcessorInformation,
        ptr::null_mut(),
        0,
        processors.as_mut_ptr() as *mut _,
        buffer_length as minwindef::ULONG,
    );

    if result == ntstatus::STATUS_SUCCESS {
        processors.set_len(proc_amount);

        Ok(processors)
    } else {
        Err(Error::last_os_error())
    }
}

pub fn frequency() -> impl Future<Item = CpuFrequency, Error = Error> {
    match unsafe { get_processors() } {
        Ok(processors) => {
            let freq = processors.into_iter().next()
                .map(CpuFrequency)
                .ok_or_else(|| Error::new(ErrorKind::UnknownValue));

            future::result(freq)
        },
        Err(e) => future::err(e)
    }
}
