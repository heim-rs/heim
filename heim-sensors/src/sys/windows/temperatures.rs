use heim_common::prelude::*;

use crate::TemperatureSensor;

pub fn temperatures() -> impl Stream<Item = Result<TemperatureSensor>> {
    // TODO: Stub
    stream::iter(vec![])
}
