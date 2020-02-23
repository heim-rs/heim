use heim_common::prelude::*;
use heim_common::units::{time, Time};
use heim_runtime as rt;

pub async fn boot_time() -> Result<Time> {
    let contents = rt::fs::read_to_string("/proc/stat").await?;

    for line in contents.lines() {
        if line.starts_with("btime ") {
            let mut parts = line.splitn(2, ' ');
            let _ = parts.next();
            return match parts.next() {
                Some(raw_value) => raw_value
                    .parse::<f64>()
                    .map(Time::new::<time::second>)
                    .map_err(Into::into),
                None => Err(Error::incompatible(
                    "Unable to parse btime value from the /proc/stat",
                )),
            };
        }
    }

    Err(Error::incompatible(
        "Unable to find btime value in the /proc/stat",
    ))
}
