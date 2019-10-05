use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};

use super::wrappers;

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

pub async fn frequency() -> Result2<CpuFrequency> {
    let current = wrappers::cpu_frequency()?;
    let min = wrappers::cpu_frequency_min()?;
    let max = wrappers::cpu_frequency_max()?;

    Ok(CpuFrequency {
        current: Frequency::new::<frequency::hertz>(current),
        min: Frequency::new::<frequency::hertz>(min),
        max: Frequency::new::<frequency::hertz>(max),
    })
}
