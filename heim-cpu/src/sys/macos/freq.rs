use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};

use super::bindings;

#[derive(Debug)]
pub struct CpuFrequency {
    current: Frequency,
    min: Frequency,
    max: Frequency,
}

impl CpuFrequency {
    pub fn current(&self) -> Frequency {
        self.current
    }

    pub fn min(&self) -> Option<Frequency> {
        Some(self.min)
    }

    pub fn max(&self) -> Option<Frequency> {
        Some(self.max)
    }
}

pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    future::lazy(|_| {
        let current = bindings::cpu_frequency()?;
        let min = bindings::cpu_frequency_min()?;
        let max = bindings::cpu_frequency_max()?;

        Ok(CpuFrequency {
            current: Frequency::new::<frequency::hertz>(current),
            min: Frequency::new::<frequency::hertz>(min),
            max: Frequency::new::<frequency::hertz>(max),
        })
    })
}
