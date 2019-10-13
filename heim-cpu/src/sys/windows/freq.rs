use std::fmt;
use std::io;

use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};

use ntapi::ntpoapi::PROCESSOR_POWER_INFORMATION;

use super::wrappers;

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

impl fmt::Debug for CpuFrequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CpuFrequency")
            .field("number", &self.0.Number)
            .field("max_mhz", &self.0.MaxMhz)
            .field("current_mhz", &self.0.CurrentMhz)
            .field("mhz_limit", &self.0.MhzLimit)
            .field("max_idle_state", &self.0.MaxIdleState)
            .field("current_idle_state", &self.0.CurrentIdleState)
            .finish()
    }
}

pub async fn frequency() -> Result<CpuFrequency> {
    let processors = wrappers::get_processors()?;

    processors
        .into_iter()
        .next()
        .map(CpuFrequency)
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound).into())
}
