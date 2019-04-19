// TODO: Implement stubs
// https://github.com/heim-rs/heim/issues/2

use heim_common::prelude::*;

use crate::units;

pub struct CpuFrequency;

impl CpuFrequency {
    pub fn current(&self) -> units::Frequency {
        unimplemented!()
    }

    pub fn min(&self) -> Option<units::Frequency> {
        unimplemented!()
    }

    pub fn max(&self) -> Option<units::Frequency> {
        unimplemented!()
    }
}

pub fn frequency() -> impl Future<Item = CpuFrequency, Error = Error> {
    future::ok(CpuFrequency)
}
