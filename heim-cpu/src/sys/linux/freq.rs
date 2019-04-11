use std::io;
use std::ops;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::si::u64::Frequency;
use heim_common::units::si::frequency::hertz;

#[derive(Debug, Default, heim_derive::Getter)]
pub struct CpuFrequency {
    current: Frequency,
    min: Option<Frequency>,
    max: Option<Frequency>,
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

        CpuFrequency {
            current,
            max,
            min,
        }
    }
}

pub fn frequency() -> impl Future<Item=CpuFrequency, Error=Error> {
    let init = CpuFrequency::default();
    frequencies().fold((init, 0u64), |(acc, amount), freq| {
        Ok::<_, Error>((acc + freq, amount + 1))
    }).map(|(mut freq, amount)| {
        freq.current /= amount;
        if let Some(min) = freq.min.as_mut() {
            *min /= amount;
        }
        if let Some(max) = freq.max.as_mut() {
            *max /= amount;
        }
        freq
    })
}

pub fn frequencies() -> impl Stream<Item=CpuFrequency, Error=Error> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But at my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    // TODO: `glob::glob` is synchronous, should replace it with `tokio::fs::readdir`
    let walker = glob::glob("/sys/devices/system/cpu/cpu[0-9]*/cpufreq")
                .expect("Invalid glob pattern");

    stream::iter_result(walker)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e).into())
        .and_then(|path| {
            let current = current_freq(&path);
            let max = max_freq(&path);
            let min = min_freq(&path);

            current.join3(max, min)
        })
        .map(|(current, max, min)| {
            CpuFrequency {
                current,
                max,
                min,
            }
        })
}

#[allow(clippy::redundant_closure)]
fn read_freq(path: PathBuf) -> impl Future<Item=Frequency, Error=Error> {
    utils::fs::read_to_string(path)
        .and_then(|value| Ok(value.trim_end().parse::<u64>()?))
        .map(|value| Frequency::new::<hertz>(value))
}

fn current_freq(path: &Path) -> impl Future<Item=Frequency, Error=Error> {
    let one = read_freq(path.join("scaling_cur_freq"));
    let two = read_freq(path.join("cpuinfo_cur_freq"));

    // TODO: Possible priority conflicts
    one.select(two)
        .map(|(freq, _)| freq)
        .map_err(|(err, _)| err)
}

#[allow(clippy::redundant_closure)]
fn max_freq(path: &Path) -> impl Future<Item=Option<Frequency>, Error=Error> {
    read_freq(path.join("scaling_max_freq"))
        .map(|freq| Some(freq))
        .or_else(|_| Ok(None))
}

#[allow(clippy::redundant_closure)]
fn min_freq(path: &Path) -> impl Future<Item=Option<Frequency>, Error=Error> {
    read_freq(path.join("scaling_min_freq"))
        .map(|freq| Some(freq))
        .or_else(|_| Ok(None))
}
