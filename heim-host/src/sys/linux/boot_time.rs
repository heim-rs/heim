use heim_common::prelude::*;
use heim_common::units::{time, Time};
use heim_runtime::fs;

pub fn boot_time() -> impl Future<Output = Result<Time>> {
    fs::read_to_string("/proc/stat")
        .map_err(Into::into)
        .and_then(|contents| {
            for line in contents.lines() {
                if line.starts_with("btime ") {
                    let mut parts = line.splitn(2, ' ');
                    let _ = parts.next();
                    let res = match parts.next() {
                        Some(raw_value) => raw_value
                            .parse::<f64>()
                            .map(Time::new::<time::second>)
                            .map_err(Into::into),
                        None => Err(Error::incompatible(
                            "Unable to parse btime value from the /proc/stat",
                        )),
                    };

                    return future::ready(res);
                }
            }

            future::err(Error::incompatible(
                "Unable to find btime value in the /proc/stat",
            ))
        })
}
