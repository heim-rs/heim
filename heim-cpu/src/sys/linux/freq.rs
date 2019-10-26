use std::io;
use std::ops;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::{future, Error, Result, Stream, TryFutureExt, TryStreamExt};
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

pub async fn frequency() -> Result<CpuFrequency> {
    let (acc, amount) = frequencies()
        .try_fold((CpuFrequency::default(), 0), |(acc, amount), frequency| {
            future::ok((acc + frequency, amount + 1))
        })
        .await?;

    // Amount could be 0 if there is an implementation bug or we are in the virtualized environment
    // and should fetch the information from some other place.

    if amount > 0 {
        Ok(CpuFrequency {
            current: acc.current / amount,
            min: acc.min.map(|value| value / amount),
            max: acc.max.map(|value| value / amount),
        })
    } else {
        let e = Error::from(io::Error::from(io::ErrorKind::InvalidData))
            .with_message("Unable to find frequencies information");
        Err(e)
    }
}

pub fn frequencies() -> impl Stream<Item = Result<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But on my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    fs::read_dir("/sys/devices/system/cpu/")
        .try_flatten_stream()
        .map_err(Error::from)
        .try_filter_map(read_frequencies)
}

/// Digging through the `/sys/devices/system/cpu/(cpu[0-9]+)/` folder
/// and searching for a frequency files, Indiana Jones style.
async fn read_frequencies(entry: fs::DirEntry) -> Result<Option<CpuFrequency>> {
    let name = entry.file_name();
    let bytes = name.as_bytes();

    // Filtering out folders not matching the `cpu[0-9]+` pattern first
    if !bytes.starts_with(b"cpu") {
        return Ok(None);
    }
    if !&bytes[3..].iter().all(|byte| *byte >= b'0' && *byte <= b'9') {
        return Ok(None);
    }

    let root = entry.path().join("cpufreq");
    let frequencies =
        future::try_join3(current_freq(&root), max_freq(&root), min_freq(&root)).await;

    match frequencies {
        Ok((current, max, min)) => Ok(Some(CpuFrequency { current, max, min })),
        // `Not found` error can happen for a `current_freq` branch,
        // which effectively means that it is not a folder we are looking for.
        // This might happen in some virtualized environments,
        // so we should just skip such errors.
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

async fn current_freq(path: &Path) -> Result<Frequency> {
    read_freq_value(path.join("scaling_cur_freq")).await

    // TODO: Add support for `cpuinfo_cur_freq` file too

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
    let res = read_freq_value(path.join("scaling_max_freq")).await;

    // We are effectively do not care about any errors here
    Ok(res.ok())
}

async fn min_freq(path: &Path) -> Result<Option<Frequency>> {
    let res = read_freq_value(path.join("scaling_min_freq")).await;

    // We are effectively do not care about any errors here
    Ok(res.ok())
}

async fn read_freq_value(path: PathBuf) -> Result<Frequency> {
    let content = fs::read_to_string(path).await?;
    let khz = content.trim_end().parse::<u64>()?;

    Ok(Frequency::new::<frequency::kilohertz>(khz))
}
