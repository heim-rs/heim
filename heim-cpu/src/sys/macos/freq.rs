use heim_common::prelude::*;

use crate::units;
use super::bindings;

pub struct CpuFrequency {
    current: units::Frequency,
    min: units::Frequency,
    max: units::Frequency,
}

impl CpuFrequency {
    pub fn current(&self) -> units::Frequency {
        self.current
    }

    pub fn min(&self) -> Option<units::Frequency> {
        Some(self.min)
    }

    pub fn max(&self) -> Option<units::Frequency> {
        Some(self.max)
    }
}

pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    future::lazy(|_| {
        let current = bindings::cpu_frequency()?;
        let min = bindings::cpu_frequency_min()?;
        let max = bindings::cpu_frequency_max()?;

        Ok(CpuFrequency {
            current: units::Frequency::new(current),
            min: units::Frequency::new(min),
            max: units::Frequency::new(max),
        })
    })
}
