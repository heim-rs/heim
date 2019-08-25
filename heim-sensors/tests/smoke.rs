use heim_common::prelude::*;
use heim_sensors as sensors;

#[heim_derive::test]
async fn smoke_temperatures() {
    let mut temperatures = sensors::temperatures();
    while let Some(sensor) = temperatures.next().await {
        let sensor = sensor.unwrap();

        let _ = sensor.unit();
        let _ = sensor.label();
        let _ = sensor.current();
        let _ = sensor.high();
        let _ = sensor.critical();
    }
}
