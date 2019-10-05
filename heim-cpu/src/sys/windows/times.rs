use std::io;

use ntapi::ntexapi::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
use winapi::shared::minwindef;
use winapi::um::processthreadsapi;

use heim_common::prelude::*;
use heim_common::sys::IntoTime as _;
use heim_common::units::Time;

use super::wrappers;

#[derive(Debug)]
pub struct CpuTime {
    user: Time,
    system: Time,
    idle: Time,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.user
    }

    pub fn system(&self) -> Time {
        self.system
    }

    pub fn idle(&self) -> Time {
        self.idle
    }
}

// https://docs.microsoft.com/en-us/windows/desktop/api/processthreadsapi/nf-processthreadsapi-getsystemtimes
pub async fn time() -> Result2<CpuTime> {
    let mut user = minwindef::FILETIME::default();
    let mut kernel = minwindef::FILETIME::default();
    let mut idle = minwindef::FILETIME::default();

    let result = unsafe { processthreadsapi::GetSystemTimes(&mut idle, &mut kernel, &mut user) };

    if result == 0 {
        // TODO: Attach error context
        Err(io::Error::last_os_error().into())
    } else {
        let user = user.into_time();
        let idle = idle.into_time();
        // Same as `psutil` subtracting idle time
        // and leaving only busy kernel time
        let system = kernel.into_time() - idle;

        Ok(CpuTime { user, system, idle })
    }
}

pub fn times() -> impl Stream<Item = Result2<CpuTime>> {
    future::lazy(|_| {
        let processors: Vec<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION> =
            wrappers::query_system_information()?;

        let stream = stream::iter(processors).map(Ok);

        Ok(stream)
    })
    .try_flatten_stream()
    .map_ok(|proc_info: SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION| {
        let user = proc_info.UserTime.into_time();
        let idle = proc_info.IdleTime.into_time();
        let system = proc_info.KernelTime.into_time() - idle;

        CpuTime { user, system, idle }
    })
}
