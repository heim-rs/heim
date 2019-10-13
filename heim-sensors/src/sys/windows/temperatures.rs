use heim_common::prelude::*;

use crate::TemperatureSensor;

pub fn temperatures() -> impl Stream<Item = Result2<TemperatureSensor>> {
    // TODO: Stub
    stream::iter(vec![])
}
