use heim_common::prelude::*;
use heim_common::units::{time, Time};
use heim_runtime as rt;

#[allow(clippy::redundant_closure)]
pub async fn uptime() -> Result<Time> {
    let contents = rt::fs::read_to_string("/proc/uptime").await?;

    match contents.splitn(2, ' ').next() {
        Some(raw_value) => {
            let seconds = raw_value.parse::<f64>()?;

            Ok(Time::new::<time::second>(seconds))
        }
        None => Err(Error::missing_entity("/proc/uptime")),
    }
}
