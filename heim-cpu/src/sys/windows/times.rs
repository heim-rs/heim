use winapi::shared::minwindef;
use winapi::um::processthreadsapi;

use super::bindings::winternl;
use heim_common::prelude::*;
use heim_common::sys::IntoTime as _;
use heim_common::units::Time;

#[derive(Debug, Clone)]
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
pub async fn time() -> Result<CpuTime> {
    let mut user = minwindef::FILETIME::default();
    let mut kernel = minwindef::FILETIME::default();
    let mut idle = minwindef::FILETIME::default();

    let result = unsafe { processthreadsapi::GetSystemTimes(&mut idle, &mut kernel, &mut user) };

    if result == 0 {
        Err(Error::last_os_error().with_ffi("GetSystemTimes"))
    } else {
        let user = user.into_time();
        let idle = idle.into_time();
        // Same as `psutil` subtracting idle time
        // and leaving only busy kernel time
        let system = kernel.into_time() - idle;

        Ok(CpuTime { user, system, idle })
    }
}

pub async fn times() -> Result<impl Stream<Item = Result<CpuTime>>> {
    let processors: Vec<winternl::SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION> =
        winternl::query_system_information()?;

    let stream = stream::iter(processors).map(|proc_info| {
        let user = proc_info.UserTime.into_time();
        let idle = proc_info.IdleTime.into_time();
        let system = proc_info.KernelTime.into_time() - idle;

        Ok(CpuTime { user, system, idle })
    });

    Ok(stream)
}
