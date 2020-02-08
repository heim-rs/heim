use std::ops;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};
use heim_runtime::fs;

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

pub fn frequency() -> impl Future<Output = Result<CpuFrequency>> {
    let init = CpuFrequency::default();
    frequencies()
        .try_fold((init, 0u64), |(acc, amount), freq| {
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
                Ok((ref freq, amount)) if amount > 0 => future::ok(CpuFrequency {
                    current: freq.current / amount,
                    min: freq.min.map(|value| value / amount),
                    max: freq.max.map(|value| value / amount),
                }),
                // Unable to determine CPU frequencies for some reasons.
                // Might happen for containerized environments, such as Microsoft Azure, for example.
                Ok(_) => future::err(Error::incompatible(
                    "No CPU frequencies was found, running in VM?",
                )),
                Err(e) => future::err(e),
            }
        })
}

pub fn frequencies() -> impl Stream<Item = Result<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But at my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    fs::read_dir("/sys/devices/system/cpu/")
        .map_err(Error::from)
        .try_filter(|entry| {
            let name = entry.file_name();
            let bytes = name.as_bytes();
            if !bytes.starts_with(b"cpu") {
                return future::ready(false);
            }
            let all_digits = &bytes[3..].iter().all(|byte| *byte >= b'0' && *byte <= b'9');

            future::ready(*all_digits)
        })
        .map_ok(|entry| entry.path().join("cpufreq"))
        .try_filter(|path| {
            // TODO: Get rid of the `.clone()`
            fs::path_exists(path.clone())
        })
        .and_then(|path| {
            let current = current_freq(&path);
            let max = max_freq(&path);
            let min = min_freq(&path);

            future::try_join3(current, max, min)
        })
        .and_then(|(current, max, min)| future::ok(CpuFrequency { current, max, min }))
}

#[allow(clippy::redundant_closure)]
fn read_freq(path: PathBuf) -> impl Future<Output = Result<Frequency>> {
    fs::read_to_string(path)
        .map_err(Error::from)
        .and_then(|value| future::ready(value.trim_end().parse::<u64>().map_err(Error::from)))
        .map_ok(Frequency::new::<frequency::kilohertz>)
}

fn current_freq(path: &Path) -> impl Future<Output = Result<Frequency>> {
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

fn max_freq(path: &Path) -> impl Future<Output = Result<Option<Frequency>>> {
    read_freq(path.join("scaling_max_freq"))
        .into_future()
        .map(|value| Ok(value.ok()))
}

fn min_freq(path: &Path) -> impl Future<Output = Result<Option<Frequency>>> {
    read_freq(path.join("scaling_min_freq"))
        .into_future()
        .map(|value| Ok(value.ok()))
}
