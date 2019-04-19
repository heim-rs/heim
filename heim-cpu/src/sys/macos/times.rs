use heim_common::prelude::*;

use crate::units;
use super::bindings;
use crate::sys::unix::CLOCK_TICKS;

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
            user: units::Time::new::<units::second>(f64::from(info.user) / ticks),
            nice: units::Time::new::<units::second>(f64::from(info.nice) / ticks),
            system: units::Time::new::<units::second>(f64::from(info.system) / ticks),
            idle: units::Time::new::<units::second>(f64::from(info.idle) / ticks),
        }
    }
}

impl From<bindings::processor_cpu_load_info> for CpuTime {
    fn from(info: bindings::processor_cpu_load_info) -> CpuTime {
        let ticks = *CLOCK_TICKS;

        CpuTime {
            user: units::Time::new::<units::second>(f64::from(info.user) / ticks),
            nice: units::Time::new::<units::second>(f64::from(info.nice) / ticks),
            system: units::Time::new::<units::second>(f64::from(info.system) / ticks),
            idle: units::Time::new::<units::second>(f64::from(info.idle) / ticks),
        }
    }
}

pub fn time() -> impl Future<Item = CpuTime, Error = Error> {
    future::lazy(|| {
        let info = unsafe { bindings::cpu_load_info()? };
        Ok(info.into())
    })
}

pub fn times() -> impl Stream<Item = CpuTime, Error = Error> {
    future::lazy(|| {
        let processors = unsafe {
            bindings::processor_load_info()?
        };

        Ok(stream::iter_ok(processors))
    })
    .flatten_stream()
    .map(Into::into)
}
