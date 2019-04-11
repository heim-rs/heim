use heim_common::prelude::*;
use heim_common::units::si::f64::Time;
use heim_common::units::si::time::second;

#[allow(clippy::redundant_closure)]
pub fn uptime() -> impl Future<Item=Time, Error=Error> {
    utils::fs::read_to_string("/proc/uptime")
        .and_then(|contents| {
            match contents.splitn(2, ' ').next() {
                Some(raw_value) => raw_value.parse::<f64>().map_err(Into::into),
                None => Err(Error::new(ErrorKind::Parse))
            }
        })
        .map(|seconds: f64| Time::new::<second>(seconds))
}
