// TODO: Implement stubs
// https://github.com/heim-rs/heim/issues/10

use heim_common::prelude::*;
use heim_common::units::Information;

pub struct IoCounters;

impl IoCounters {
    pub fn device_name(&self) -> &str {
        unimplemented!()
    }

    pub fn read_count(&self) -> u64 {
        unimplemented!()
    }

    pub fn write_count(&self) -> u64 {
        unimplemented!()
    }

    pub fn read_bytes(&self) -> Information {
        unimplemented!()
    }

    pub fn write_bytes(&self) -> Information {
        unimplemented!()
    }
}

pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    stream::iter(vec![])
}

pub fn io_counters_physical() -> impl Stream<Item = Result<IoCounters>> {
    stream::iter(vec![])
}
