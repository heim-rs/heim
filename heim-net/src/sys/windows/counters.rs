use heim_common::prelude::*;
use heim_common::units::Information;
use heim_common::Pid;

#[derive(Debug)]
pub struct IoCounters;

impl IoCounters {
    pub fn interface(&self) -> &str {
        unimplemented!()
    }

    pub fn bytes_sent(&self) -> Information {
        unimplemented!()
    }

    pub fn bytes_recv(&self) -> Information {
        unimplemented!()
    }

    pub fn packets_sent(&self) -> u64 {
        unimplemented!()
    }

    pub fn packets_recv(&self) -> u64 {
        unimplemented!()
    }

    pub fn errors_sent(&self) -> u64 {
        unimplemented!()
    }

    pub fn errors_recv(&self) -> u64 {
        unimplemented!()
    }

    pub fn drop_recv(&self) -> u64 {
        unimplemented!()
    }

    pub fn drop_sent(&self) -> u64 {
        unimplemented!()
    }
}

pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    stream::iter(vec![])
}

pub fn io_counters_for_pid(_pid: Pid) -> impl Stream<Item = Result<IoCounters>> {
    stream::iter(vec![])
}
