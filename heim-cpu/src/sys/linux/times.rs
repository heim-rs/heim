use std::str::{self, FromStr};

use heim_common::prelude::*;
use heim_common::units::si::f64::Time;
use heim_common::units::si::time::second;

use super::CLOCK_TICKS;

#[derive(Debug, Default, heim_derive::Getter)]
pub struct CpuTime {
    user: Time,
    nice: Time,
    system: Time,
    idle: Time,
    io_wait: Time,
    irq: Time,
    soft_irq: Time,
    steal: Option<Time>,
    guest: Option<Time>,
    guest_nice: Option<Time>,
}

impl FromStr for CpuTime {
    type Err = Error;

    // Parse one line from the /proc/stat, ex.
    // "cpu1 317865 456 71065 3101075 8645 14938 10567 0 0 0"
    fn from_str(value: &str) -> Result<CpuTime> {
        let mut times = CpuTime::default();

        let parts = value.split_whitespace().skip(1);
        for (idx, part) in parts.enumerate() {
            let value = part.parse::<u32>()
                .map(|value| {
                    let value = f64::from(value) / *CLOCK_TICKS;
                    Time::new::<second>(value)
                })?;

            match idx {
                0 => times.user = value,
                1 => times.nice = value,
                2 => times.system = value,
                3 => times.idle = value,
                4 => times.io_wait = value,
                5 => times.irq = value,
                6 => times.soft_irq = value,
                7 => times.steal = Some(value),
                8 => times.guest = Some(value),
                9 => times.guest_nice = Some(value),
                _ => break,
            };
        }

        Ok(times)
    }
}

pub fn time() -> impl Future<Item=CpuTime, Error=Error> {
    // cumulative time is always the first line
    utils::fs::read_lines_into::<_, CpuTime, _>("/proc/stat")
        .take(1)
        .into_future()
        .map_err(|(e, _)| e)
        .and_then(|(time, _)| match time {
            Some(time) => Ok(time),
            None => Err(Error::new(ErrorKind::Parse))
        })
}

#[allow(clippy::redundant_closure)]
pub fn times() -> impl Stream<Item=CpuTime, Error=Error> {
    utils::fs::read_lines("/proc/stat")
        .skip(1)
        .filter(|line| line.starts_with("cpu"))  // TODO: Check if bytes comparision would be faster
        .and_then(|line| CpuTime::from_str(&line))
}
