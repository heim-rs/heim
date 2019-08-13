use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;

use crate::units;
use super::bindings;

#[derive(Debug)]
pub struct CpuTime {
    user: units::Time,
    nice: units::Time,
    system: units::Time,
    idle: units::Time,
}

impl CpuTime {
    pub fn user(&self) -> units::Time {
        self.user
    }

    pub fn nice(&self) -> units::Time {
        self.nice
    }

    pub fn system(&self) -> units::Time {
        self.system
    }

    pub fn idle(&self) -> units::Time {
        self.idle
    }
}

impl From<bindings::host_cpu_load_info> for CpuTime {
    fn from(info: bindings::host_cpu_load_info) -> CpuTime {
        let ticks = *CLOCK_TICKS;

        CpuTime {
            user: units::Time::new(f64::from(info.user) / ticks),
            nice: units::Time::new(f64::from(info.nice) / ticks),
            system: units::Time::new(f64::from(info.system) / ticks),
            idle: units::Time::new(f64::from(info.idle) / ticks),
        }
    }
}

impl From<bindings::processor_cpu_load_info> for CpuTime {
    fn from(info: bindings::processor_cpu_load_info) -> CpuTime {
        let ticks = *CLOCK_TICKS;

        CpuTime {
            user: units::Time::new(f64::from(info.user) / ticks),
            nice: units::Time::new(f64::from(info.nice) / ticks),
            system: units::Time::new(f64::from(info.system) / ticks),
            idle: units::Time::new(f64::from(info.idle) / ticks),
        }
    }
}

pub fn time() -> impl Future<Output = Result<CpuTime>> {
    future::lazy(|_| {
        let info = unsafe { bindings::cpu_load_info()? };
        Ok(info.into())
    })
}

pub fn times() -> impl Stream<Item = Result<CpuTime>> {
    future::lazy(|_| {
        let processors = unsafe {
            bindings::processor_load_info()?
        };

        let stream = stream::iter(processors).map(Ok);

        Ok(stream)
    })
    .try_flatten_stream()
    .map_ok(Into::into)
}
