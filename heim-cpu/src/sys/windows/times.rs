use std::ptr;
use std::mem;

use winapi::shared::minwindef;
use winapi::um::processthreadsapi;

use heim_common::prelude::*;

use crate::units;
use super::bindings::{get_system_info, winternl, IntoTime};

pub struct CpuTime {
    user: units::Time,
    system: units::Time,
    idle: units::Time,
}

impl CpuTime {
    pub fn user(&self) -> units::Time {
        self.user
    }

    pub fn system(&self) -> units::Time {
        self.system
    }

    pub fn idle(&self) -> units::Time {
        self.idle
    }
}

// https://docs.microsoft.com/en-us/windows/desktop/api/processthreadsapi/nf-processthreadsapi-getsystemtimes
pub fn time() -> impl Future<Item = CpuTime, Error = Error> {
    let mut user = minwindef::FILETIME::default();
    let mut kernel = minwindef::FILETIME::default();
    let mut idle = minwindef::FILETIME::default();

    let result = unsafe {
        processthreadsapi::GetSystemTimes(
            &mut idle,
            &mut kernel,
            &mut user,
        )
    };

    if result == 0 {
        future::err(Error::last_os_error())
    } else {
        let user = user.into_time();
        let idle = idle.into_time();
        // Same as `psutil` subtracting idle tim
        // and leaving only busy kernel time
        let system = kernel.into_time() - idle;

        future::ok(CpuTime {
            user,
            system,
            idle,
        })
    }
}

pub fn times() -> impl Stream<Item = CpuTime, Error = Error> {
    future::lazy(|| {
        let info = unsafe { get_system_info() };
        let proc_amount = info.dwNumberOfProcessors as usize;
        let mut processors = Vec::<winternl::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>::with_capacity(proc_amount);
        let buffer_length = proc_amount * mem::size_of::<winternl::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>();

        unsafe {
            winternl::NtQuerySystemInformation(
                winternl::SystemProcessorPerformanceInformation,
                processors.as_mut_ptr() as *mut _,
                buffer_length as u32,
                ptr::null_mut(),
            )?;
            processors.set_len(proc_amount);
        };

        Ok(stream::iter_ok(processors))
    })
    .flatten_stream()
    .map(|proc_info| {
        let user = proc_info.UserTime.into_time();
        let idle = proc_info.IdleTime.into_time();
        let system = proc_info.KernelTime.into_time() - idle;

        CpuTime {
            user,
            system,
            idle,
        }
    })
}
