use std::ffi::OsStr;
use std::ops;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
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

pub async fn frequency() -> Result<CpuFrequency> {
    let init = CpuFrequency::default();
    frequencies()
        .try_fold((init, 0u64), |(acc, amount), freq| {
            future::ok((acc + freq, amount + 1))
        })
        .then(|result| {
            match result {
                // Will panic here if `frequencies()` stream returns nothing,
                // which is either a bug in implementation or we are in container
                // and should fetch information from another place.
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
        .await
}

/// Check if file name matches the `cpu\d+` mask.
fn cpu_match(filename: &OsStr) -> bool {
    let bytes = filename.as_bytes();
    if !bytes.starts_with(b"cpu") {
        return false;
    }

    (&bytes[3..])
        .iter()
        .all(|byte| *byte >= b'0' && *byte <= b'9')
}

pub fn frequencies() -> impl Stream<Item = Result<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But at my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    rt::fs::read_dir("/sys/devices/system/cpu/")
        .try_flatten_stream()
        .map_err(Error::from)
        .try_filter_map(|entry| async move {
            if !cpu_match(&entry.file_name()) {
                Ok(None)
            } else {
                // Note: at this point we are not doing `.await`
                // in order to execute generated futures in a parallel later
                // with help of `.try_buffer_unordered`
                Ok(Some(core_frequency(entry.path().join("cpufreq"))))
            }
        })
        // Let's assume for a while that there will be at least 4 logical cores
        .try_buffer_unordered(4)
        .try_filter_map(|result| async {
            Ok(result)
        })
}

/// Load frequency files from one CPU core.
///
/// `root` points to directory like `"/sys/devices/system/cpu/cpu0/cpufreq/"`.
///
/// ## Returns
///
/// If `Ok(None)` is returned, it is not a CPU core directory
/// and the results should be ignored.
async fn core_frequency(root: PathBuf) -> Result<Option<CpuFrequency>> {
    // TODO: This thing seems to be unnecessary
    // One option is to check `Err(NotFound)` at the match below,
    // but it needs to be investigated first and probably covered by a test.
    if !rt::fs::path_exists(&root).await {
        return Ok(None);
    }

    let frequencies = rt::try_join!(current_freq(&root), max_freq(&root), min_freq(&root));
    match frequencies {
        Ok((current, max, min)) => Ok(Some(CpuFrequency { current, max, min })),
        Err(e) => Err(e),
    }
}

#[allow(clippy::redundant_closure)]
async fn read_freq(path: PathBuf) -> Result<Frequency> {
    let contents = rt::fs::read_to_string(path).await?;
    let value = contents.trim_end().parse::<u64>()?;

    Ok(Frequency::new::<frequency::kilohertz>(value))
}

async fn current_freq(path: &Path) -> Result<Frequency> {
    read_freq(path.join("scaling_cur_freq")).await

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

async fn max_freq(path: &Path) -> Result<Option<Frequency>> {
    let value = read_freq(path.join("scaling_max_freq")).await;

    // Don't care about errors propagation at this point
    Ok(value.ok())
}

async fn min_freq(path: &Path) -> Result<Option<Frequency>> {
    let value = read_freq(path.join("scaling_min_freq")).await;

    // Don't care about errors propagation at this point
    Ok(value.ok())
}
