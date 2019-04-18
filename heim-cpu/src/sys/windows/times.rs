// TODO: Implement stubs
// https://github.com/heim-rs/heim/issues/5
// https://github.com/heim-rs/heim/issues/7

use heim_common::prelude::*;

use crate::units;

pub struct CpuTime;

impl CpuTime {
    pub fn user(&self) -> units::Time {
        unimplemented!()
    }

    pub fn system(&self) -> units::Time {
        unimplemented!()
    }

    pub fn idle(&self) -> units::Time {
        unimplemented!()
    }
}

pub fn time() -> impl Future<Item = CpuTime, Error = Error> {
    future::ok(CpuTime)
}

pub fn times() -> impl Stream<Item = CpuTime, Error = Error> {
    stream::iter_ok(vec![])
}
