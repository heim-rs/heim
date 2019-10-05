use std::io;
use std::ops;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use heim_common::prelude::*;
use heim_common::units::{frequency, Frequency};
use heim_runtime::fs;

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

        CpuFrequency { current, max, min }
    }
}

pub async fn frequency() -> Result2<CpuFrequency> {
    let mut acc = CpuFrequency::default();
    let mut amount = 0;
    let frequencies = frequencies();
    pin_utils::pin_mut!(frequencies);

    while let Some(frequency) = frequencies.next().await {
        acc = acc + frequency?;
        amount += 1;
    }

    // Amount could be 0 if there is an implementation bug or we are in the virtualized environment
    // and should fetch the information from some other place.

    if amount > 0 {
        Ok(CpuFrequency {
            current: acc.current / amount,
            min: acc.min.map(|value| value / amount),
            max: acc.max.map(|value| value / amount),
        })
    } else {
        // TODO: Attach error context
        Err(io::Error::from(io::ErrorKind::InvalidData).into())
    }
}

pub fn frequencies() -> impl Stream<Item = Result2<CpuFrequency>> {
    // TODO: psutil looks into `/sys/devices/system/cpu/cpufreq/policy*` at first
    // But on my machine with Linux 5.0 `./cpu/cpu*/cpufreq` are symlinks to the `policy*`,
    // so at least we will cover most cases in first iteration and will fix weird values
    // later with the thoughts and patches

    // TODO: https://github.com/giampaolo/psutil/issues/1269

    fs::read_dir("/sys/devices/system/cpu/")
        .map_err(Error2::from)
        .try_filter_map(read_frequencies)
}

/// Digging through the `/sys/devices/system/cpu/(cpu[0-9]+)/` folder
/// and searching for a frequency files, Indiana Jones style.
async fn read_frequencies(entry: fs::DirEntry) -> Result2<Option<CpuFrequency>> {
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

    let (current, max, min) =
        future::try_join3(current_freq(&root), max_freq(&root), min_freq(&root)).await?;

    Ok(Some(CpuFrequency { current, max, min }))
}

async fn current_freq(path: &Path) -> Result2<Frequency> {
    // TODO: Wait for Future' `try_select_all` and uncomment the block below
    // Ref: https://github.com/rust-lang-nursery/futures-rs/pull/1557

    read_freq_value(path.join("scaling_cur_freq")).await

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

async fn max_freq(path: &Path) -> Result2<Option<Frequency>> {
    let res = read_freq_value(path.join("scaling_max_freq")).await;

    // We are effectively do not care about any errors here
    Ok(res.ok())
}

async fn min_freq(path: &Path) -> Result2<Option<Frequency>> {
    let res = read_freq_value(path.join("scaling_min_freq")).await;

    // We are effectively do not care about any errors here
    Ok(res.ok())
}

async fn read_freq_value(path: PathBuf) -> Result2<Frequency> {
    let content = fs::read_to_string(path).await?;
    let khz = content.trim_end().parse::<u64>()?;

    Ok(Frequency::new::<frequency::kilohertz>(khz))
}
