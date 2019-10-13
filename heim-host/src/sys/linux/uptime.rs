use std::io;

use heim_common::prelude::*;
use heim_common::units::{time, Time};
use heim_runtime::fs;

pub async fn uptime() -> Result<Time> {
    let contents = fs::read_to_string("/proc/uptime").await?;

    match contents.splitn(2, ' ').next() {
        Some(raw_value) => {
            let seconds = raw_value.parse::<f64>()?;

            Ok(Time::new::<time::second>(seconds))
        }
        None => {
            let e = Error::from(io::Error::from(io::ErrorKind::InvalidData))
                .with_message("Unable to parse /proc/uptime");

            Err(e)
        }
    }
}
