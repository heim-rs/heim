use std::io;
use std::str::{self, FromStr};

use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;
use heim_common::units::{time, Time};
use heim_runtime::fs;

#[derive(Debug, Default, heim_derive::Getter)]
pub struct CpuTime {
    user: Time,
    nice: Time,
    system: Time,
    idle: Time,
    io_wait: Time,
    irq: Time,
    soft_irq: Time,
    steal: Time,
    guest: Option<Time>,
    guest_nice: Option<Time>,
}

impl FromStr for CpuTime {
    type Err = Error2;

    // Parse one line from the /proc/stat, ex.
    // "cpu1 317865 456 71065 3101075 8645 14938 10567 0 0 0"
    fn from_str(value: &str) -> Result2<CpuTime> {
        let mut times = CpuTime::default();

        let parts = value.split_whitespace().skip(1);
        for (idx, part) in parts.enumerate() {
            let value = part.parse::<u32>().map(|value| {
                let value = f64::from(value) / *CLOCK_TICKS;
                Time::new::<time::second>(value)
            })?;

            match idx {
                0 => times.user = value,
                1 => times.nice = value,
                2 => times.system = value,
                3 => times.idle = value,
                4 => times.io_wait = value,
                5 => times.irq = value,
                6 => times.soft_irq = value,
                7 => times.steal = value,
                8 => times.guest = Some(value),
                9 => times.guest_nice = Some(value),
                _ => break,
            };
        }

        Ok(times)
    }
}

pub async fn time() -> Result2<CpuTime> {
    let mut lines = fs::read_lines_into::<_, CpuTime, _>("/proc/stat");
    // cumulative time is always the first line
    match lines.next().await {
        Some(Ok(time)) => Ok(time),
        Some(Err(e)) => Err(e),
        // TODO: Attach error context
        None => Err(io::Error::from(io::ErrorKind::InvalidData).into()),
    }
}

pub fn times() -> impl Stream<Item = Result2<CpuTime>> {
    fs::read_lines("/proc/stat")
        .skip(1)
        .try_filter(|line| future::ready(line.starts_with("cpu")))
        .map_err(Error2::from)
        .and_then(|line| future::ready(CpuTime::from_str(&line)))
}
