use std::io;
use std::ops;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;

use crate::units::Frequency;

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

pub fn frequency() -> impl Future<Output=Result<CpuFrequency>> {
    let init = CpuFrequency::default();
    frequencies().try_fold((init, 0u64), |(acc, amount), freq| {
        future::ok((acc + freq, amount + 1))
    })
    .then(|result| {
        match result {
            // Will panic here if `frequencies()` stream returns nothing,
            // which is either a bug in implementation or we are in container
            // and should fetch information from the another place.
            //
            // Also, `bind_by_move_pattern_guards` feature
            // would simplify the following code a little,
            // `freq` can be modified and returned in place
            Ok((ref freq, amount)) if amount > 0 => {
                future::ok(CpuFrequency {
                    current: freq.current / amount,
                    min: freq.min.map(|value| value / amount),
                    max: freq.max.map(|value| value / amount),
                })
            },
            // Unable to determine CPU frequencies for some reasons.
            // Might happen for containerized environments, such as Microsoft Azure, for example.
            Ok(_) => future::err(Error::new(ErrorKind::UnknownValue("No data"))),
            Err(e) => future::err(e),
        }
    })
}

pub fn frequencies() -> impl Stream<Item=Result<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But at my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    // TODO: `glob::glob` is synchronous, should replace it with some async dir reader
    let walker = glob::glob("/sys/devices/system/cpu/cpu[0-9]*/cpufreq")
                .expect("Invalid glob pattern");

    stream::iter(walker)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e).into())
        .and_then(|path| {
            let current = current_freq(&path);
            let max = max_freq(&path);
            let min = min_freq(&path);

            future::try_join3(current, max, min)
        })
        .and_then(|(current, max, min)| {
            future::ok(CpuFrequency {
                current,
                max,
                min,
            })
        })
}

#[allow(clippy::redundant_closure)]
fn read_freq(path: PathBuf) -> impl Future<Output=Result<Frequency>> {
    utils::fs::read_to_string(path)
        .and_then(|value| {
            future::ready(
                value.trim_end().parse::<u64>().map_err(Error::from)
            )
        })
        .map_ok(Frequency::new)
}

fn current_freq(path: &Path) -> impl Future<Output=Result<Frequency>> {
    // TODO: Wait for Future' `try_select_all` and uncomment the block below
    // Ref: https://github.com/rust-lang-nursery/futures-rs/pull/1557

    read_freq(path.join("scaling_cur_freq"))

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

fn max_freq(path: &Path) -> impl Future<Output=Result<Option<Frequency>>> {
    read_freq(path.join("scaling_max_freq"))
        .into_future()
        .map(|value| Ok(value.ok()))
}

fn min_freq(path: &Path) -> impl Future<Output=Result<Option<Frequency>>> {
    read_freq(path.join("scaling_min_freq"))
        .into_future()
        .map(|value| Ok(value.ok()))
}
