use heim_common::prelude::*;

use crate::TemperatureSensor;

mod hwmon;
mod sysfs;

// TODO: See `temperatures()` inner comment
//mod thermal;

pub fn temperatures() -> impl Stream<Item = Result2<TemperatureSensor>> {
    hwmon::hwmon("/sys/class/hwmon/")

    // TODO: Combined stream generates huge type which overflows the Rust type size limit
    // Seems like a nightly-2019-10-13 issue, should check if it will be resolved later

    // We need the `thermal_zone` items only if `hwmon` stream yielded nothing
    //    hwmon.choose_chain(thermal)
}
