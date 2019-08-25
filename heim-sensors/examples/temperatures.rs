use heim_common::prelude::*;
use heim_sensors as sensors;

#[heim_derive::main]
async fn main() -> Result<()> {
    let mut sensors = sensors::temperatures();
    while let Some(sensor) = sensors.next().await {
        dbg!(sensor?);
    }

    Ok(())
}
