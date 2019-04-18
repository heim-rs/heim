// TODO: Implement stubs
// https://github.com/heim-rs/heim/issues/3

use heim_common::prelude::*;

pub struct CpuStats;

impl CpuStats {
    pub fn ctx_switches(&self) -> u64 {
        unimplemented!()
    }

    pub fn interrupts(&self) -> u64 {
        unimplemented!()
    }

    pub fn soft_interrupts(&self) -> u64 {
        unimplemented!()
    }
}

pub fn stats() -> impl Future<Item = CpuStats, Error = Error> {
    future::ok(CpuStats)
}
