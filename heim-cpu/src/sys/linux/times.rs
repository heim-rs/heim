use std::str::{self, FromStr};

use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;
use heim_common::units::{time, Time};
use heim_runtime as rt;

#[derive(Debug, Default)]
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

impl CpuTime {
    pub fn user(&self) -> Time {
        self.user
    }
    pub fn nice(&self) -> Time {
        self.nice
    }
    pub fn system(&self) -> Time {
        self.system
    }
    pub fn idle(&self) -> Time {
        self.idle
    }
    pub fn io_wait(&self) -> Time {
        self.io_wait
    }
    pub fn irq(&self) -> Time {
        self.irq
    }
    pub fn soft_irq(&self) -> Time {
        self.soft_irq
    }
    pub fn steal(&self) -> Time {
        self.steal
    }
    pub fn guest(&self) -> Option<Time> {
        self.guest
    }
    pub fn guest_nice(&self) -> Option<Time> {
        self.guest_nice
    }
}

impl FromStr for CpuTime {
    type Err = Error;

    // Parse one line from the /proc/stat, ex.
    // "cpu1 317865 456 71065 3101075 8645 14938 10567 0 0 0"
    fn from_str(value: &str) -> Result<CpuTime> {
        let mut times = CpuTime::default();
        let ticks = *CLOCK_TICKS as f64;

        let parts = value.split_whitespace().skip(1);
        for (idx, part) in parts.enumerate() {
            let value = part.parse::<f64>().map(|value| {
                // TODO: Potential precision loss.
                // Do we care about it at all?
                Time::new::<time::second>(value / ticks)
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

pub async fn time() -> Result<CpuTime> {
    // cumulative time is always the first line
    let mut lines = rt::fs::read_lines_into::<_, CpuTime, _>("/proc/stat").await?;
    match lines.next().await {
        Some(line) => line,
        None => Err(Error::missing_key("cumulative time line", "/proc/stat")),
    }
}

pub async fn times() -> Result<impl Stream<Item = Result<CpuTime>>> {
    let lines = rt::fs::read_lines("/proc/stat").await?;

    let stream = lines
        .skip(1)
        .filter_map(|try_line| async move {
            match try_line {
                Ok(line) if line.starts_with("cpu") => Some(CpuTime::from_str(&line)),
                Ok(..) => None,
                Err(e) => Some(Err(e.into())),
            }
        });

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::CpuTime;

    #[test]
    fn test_issue_233() {
        const LINE: &str = "cpu  465552918 3813058 111153634 9065060137 5821166 0 4346876 0 0 0\n";

        let _ = CpuTime::from_str(LINE).unwrap();
    }
}
