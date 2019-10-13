use heim_common::prelude::*;
use heim_sensors as sensors;

#[heim_derive::main]
async fn main() -> Result2<()> {
    let sensors = sensors::temperatures();
    pin_utils::pin_mut!(sensors);
    while let Some(sensor) = sensors.next().await {
        dbg!(sensor?);
    }

    Ok(())
}
