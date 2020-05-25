use std::fs;
use std::io;
use std::ops;
use std::path::{Path, PathBuf};

use heim_common::prelude::{Error, Result, Stream};
use heim_common::units::{frequency, Frequency};
use heim_runtime as rt;

#[derive(Debug, Default)]
pub struct CpuFrequency {
    current: Frequency,
    min: Option<Frequency>,
    max: Option<Frequency>,
}

impl CpuFrequency {
    pub fn current(&self) -> Frequency {
        self.current
    }

    pub fn min(&self) -> Option<Frequency> {
        self.min
    }

    pub fn max(&self) -> Option<Frequency> {
        self.max
    }
}

impl ops::Add<CpuFrequency> for CpuFrequency {
    type Output = CpuFrequency;

    fn add(self, rhs: CpuFrequency) -> CpuFrequency {
        let current = self.current + rhs.current;
        let min = match (self.min, rhs.min) {
            (Some(left), Some(right)) => Some(left + right),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        };
        let max = match (self.max, rhs.max) {
            (Some(left), Some(right)) => Some(left + right),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        };

        CpuFrequency { current, max, min }
    }
}

/// Internal blocking iterator over CPU frequencies.
fn _frequencies() -> impl Iterator<Item = Result<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But at my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269
    let entries =
        glob::glob("/sys/devices/system/cpu/cpu[0-9]/cpufreq/").expect("Incorrect glob pattern");

    entries.map(|try_path| {
        let path = try_path.map_err(|e| e.into_error())?;

        let current = current_freq(&path)?;
        let max = max_freq(&path)?;
        let min = min_freq(&path)?;

        Ok(CpuFrequency { current, max, min })
    })
}

pub fn frequencies() -> impl Stream<Item = Result<CpuFrequency>> {
    smol::iter(_frequencies())
}

pub async fn frequency() -> Result<CpuFrequency> {
    rt::spawn_blocking(|| {
        let mut acc = CpuFrequency::default();
        let mut amount = 0;
        for freq in _frequencies() {
            let freq = freq?;

            acc = acc + freq;
            amount += 1;
        }

        if amount > 0 {
            Ok(CpuFrequency {
                current: acc.current / amount,
                min: acc.min.map(|value| value / amount),
                max: acc.max.map(|value| value / amount),
            })
        } else {
            let inner = io::Error::from(io::ErrorKind::InvalidData);
            Err(Error::from(inner).with_message("No CPU frequencies was found, running in VM?"))
        }
    })
    .await
}

#[allow(clippy::redundant_closure)]
fn read_freq(path: PathBuf) -> Result<Frequency> {
    let contents = fs::read_to_string(path)?;
    let value = contents.trim_end().parse::<u64>()?;

    Ok(Frequency::new::<frequency::kilohertz>(value))
}

fn current_freq(path: &Path) -> Result<Frequency> {
    read_freq(path.join("scaling_cur_freq"))

    // TODO: Use `try_join` here instead of the code above
    //    let one = read_freq(path.join("scaling_cur_freq"))
    //        .into_future().fuse();
    //    let two = read_freq(path.join("cpuinfo_cur_freq"))
    //        .into_future().fuse();
    //
    //    let result = futures::select! {
    //        Ok(freq) = one => Ok(freq),
    //        Ok(freq) = two => Ok(freq),
    //    };
    //
    //    future::ready(result)
}

fn max_freq(path: &Path) -> Result<Option<Frequency>> {
    let value = read_freq(path.join("scaling_max_freq"));

    // Don't care about errors propagation at this point
    Ok(value.ok())
}

fn min_freq(path: &Path) -> Result<Option<Frequency>> {
    let value = read_freq(path.join("scaling_min_freq"));

    // Don't care about errors propagation at this point
    Ok(value.ok())
}
