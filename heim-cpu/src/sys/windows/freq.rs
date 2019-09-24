use std::mem;
use std::ptr;

use winapi::shared::{minwindef, ntstatus};
use winapi::um::{powerbase, winnt};

use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};

use super::bindings::get_system_info;
use super::bindings::power::PROCESSOR_POWER_INFORMATION;

#[derive(Debug)]
pub struct CpuFrequency(PROCESSOR_POWER_INFORMATION);

impl CpuFrequency {
    pub fn current(&self) -> Frequency {
        Frequency::new::<frequency::megahertz>(self.0.CurrentMhz.into())
    }

    pub fn max(&self) -> Option<Frequency> {
        Some(Frequency::new::<frequency::megahertz>(self.0.MaxMhz.into()))
    }

    pub fn min(&self) -> Option<Frequency> {
        None
    }
}

unsafe fn get_processors() -> Result<Vec<PROCESSOR_POWER_INFORMATION>> {
    let info = get_system_info();
    if info.dwNumberOfProcessors == 0 {
        return Err(Error::incompatible("No processors was found"));
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

pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    match unsafe { get_processors() } {
        Ok(processors) => {
            let freq = processors
                .into_iter()
                .next()
                .map(CpuFrequency)
                .ok_or_else(|| Error::incompatible("No processors was found"));

            future::ready(freq)
        }
        Err(e) => future::err(e),
    }
}
