use std::str::FromStr;
use std::ffi::{CString, OsStr};

use heim_common::prelude::*;
use heim_common::utils::iter::*;
use heim_common::units::{Information, Time};

// Copied from the `psutil` sources:
//
// "man iostat" states that sectors are equivalent with blocks and have
// a size of 512 bytes. Despite this value can be queried at runtime
// via /sys/block/{DISK}/queue/hw_sector_size and results may vary
// between 1k, 2k, or 4k... 512 appears to be a magic constant used
// throughout Linux source code:
// * https://stackoverflow.com/a/38136179/376587
// * https://lists.gt.net/linux/kernel/2241060
// * https://github.com/giampaolo/psutil/issues/1305
// * https://github.com/torvalds/linux/blob/4f671fe2f9523a1ea206f63fe60a7c7b3a56d5c7/include/linux/bio.h#L99
// * https://lkml.org/lkml/2015/8/17/234
const DISK_SECTOR_SIZE: u64 = 512;


#[derive(Debug, Default, heim_derive::Getter)]
pub struct IoCounters {
    #[getter(as_str)]
    name: String,
    read_count: u64,
    write_count: u64,
    read_bytes: Information,
    write_bytes: Information,
    busy_time: Time,
    read_merged_count: u64,
    write_merged_count: u64,
}

impl IoCounters {
    pub fn device_name(&self) -> &OsStr {
        OsStr::new(self.name.as_str())
    }

    // Based on the sysstat code:
    // https://github.com/sysstat/sysstat/blob/1c711c1fd03ac638cfc1b25cdf700625c173fd2c/common.c#L200
    fn is_storage_device(&self) -> impl Future<Output=bool> {
        let path = CString::new(format!("/sys/block/{}", self.name.replace("/", "!")))
            // FIXME: propagate error
            .expect("Malformed device path");

        future::lazy(move |_| {
            let result = unsafe {
                libc::access(path.as_ptr(), libc::F_OK)
            };

            result == 0
        })
    }

}

impl FromStr for IoCounters {
    type Err = Error;

    // At the moment supports format used in Linux 2.6+,
    // except ignoring discard values introduced in Linux 4.18.
    //
    // https://www.kernel.org/doc/Documentation/iostats.txt
    // https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
    fn from_str(s: &str) -> Result<IoCounters> {
        let mut parts = s.split_whitespace().skip(2);

        let name: String = parts.try_from_next()?;
        let read_count = parts.try_parse_next()?;
        let read_merged_count = parts.try_parse_next()?;
        let read_bytes = parts.try_parse_next()
            .map(|bytes: u64| Information::new(bytes * DISK_SECTOR_SIZE))?;
        let mut parts = parts.skip(1);
        let write_count = parts.try_parse_next()?;
        let write_merged_count = parts.try_parse_next()?;
        let write_bytes = parts.try_parse_next()
            .map(|bytes: u64| Information::new(bytes * DISK_SECTOR_SIZE))?;
        let mut parts = parts.skip(2);
        let busy_time = parts.try_parse_next()
            .map(|seconds: u64| Time::new(seconds as f64))?;

        Ok(IoCounters {
            name,
            read_count,
            read_merged_count,
            read_bytes,
            write_count,
            write_merged_count,
            write_bytes,
            busy_time,
        })
    }
}

pub fn io_counters() -> impl Stream<Item=Result<IoCounters>> {
    utils::fs::read_lines_into("/proc/diskstats")
        .into_stream()
}

pub fn io_counters_physical() -> impl Stream<Item=Result<IoCounters>> {
    io_counters()
        .and_then(|device| {
            device.is_storage_device()
                .map(|flag| {
                    Ok((flag, device))
                })
        })
        .try_filter(|(is_storage_device, _)| future::ready(*is_storage_device))
        .map_ok(|(_, device)| device)
}
