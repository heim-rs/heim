use std::fs;
use std::path::PathBuf;
use std::os::windows::io::AsRawHandle;

use heim_common::prelude::*;
use heim_common::units::{Time, Information};

use super::bindings::disks;
use super::bindings::volumes::Volumes;

pub struct IoCounters {
    volume_path: PathBuf,
    read_count: u64,
    write_count: u64,
    read_bytes: Information,
    write_bytes: Information,
    read_time: Time,
    write_time: Time,
}

impl IoCounters {
    pub fn device_name(&self) -> &str {
        self.volume_path.to_str()
            .expect("Volume GUID path should be the proper unicode")
    }

    pub fn read_count(&self) -> u64 {
        self.read_count
    }

    pub fn write_count(&self) -> u64 {
        self.write_count
    }

    pub fn read_bytes(&self) -> Information {
        self.read_bytes
    }

    pub fn write_bytes(&self) -> Information {
        self.write_bytes
    }

    pub fn read_time(&self) -> Time {
        self.read_time
    }

    pub fn write_time(&self) -> Time {
        self.write_time
    }
}

fn inner_stream<F>(mut filter: F) -> impl Stream<Item=Result<IoCounters>>
        where F: FnMut(&PathBuf) -> bool + 'static {
    stream::iter(Volumes::new())
    .try_filter(move |path| {
        future::ready(filter(&path))
    })
    .and_then(|mut volume_path| {
        let res = fs::File::open(&volume_path)
            // Since trailing backslash was trimmed by `Volumes` iterator,
            // we need to get it back in order to display
            // it later via `IoCounters::device_name`.
            .map(|file| {
                volume_path.push("\\");
                (volume_path, file)
            })
            .map_err(Error::from);

        future::ready(res)
    })
    .and_then(|(volume_path, file)| {
        let handle = file.as_raw_handle();

        // psutil additionally checks for some errors
        // and silently skips these disks.
        // Not sure if it will happen here, because we are working
        // with volumes instead of disks (as in `C:\\`).
        //
        // If it will happen, though, submit an issue, please.
        //
        // See: https://github.com/giampaolo/psutil/blob/c0aba35a78649c453f0c89ab163a58a8efb4639e/psutil/_psutil_windows.c#L2262-L2281

        let perf = unsafe {
            match disks::disk_performance(&handle) {
                Ok(perf) => perf,
                Err(e) => return future::err(e),
            }
        };

        let read_bytes = unsafe {
            *perf.BytesRead.QuadPart() as u64
        };
        let write_bytes = unsafe {
            *perf.BytesWritten.QuadPart() as u64
        };
        let read_time = unsafe {
            *perf.ReadTime.QuadPart() as f64
        };
        let write_time = unsafe {
            *perf.WriteTime.QuadPart() as f64
        };

        let counters = IoCounters {
            volume_path,
            read_count: u64::from(perf.ReadCount),
            write_count: u64::from(perf.WriteCount),
            read_bytes: Information::new(read_bytes),
            write_bytes: Information::new(write_bytes),
            // `ReadTime` and `WriteTime` seems to be in tenths of microseconds
            // https://github.com/giampaolo/psutil/issues/1012
            read_time: Time::from_microseconds(read_time * 10.0),
            write_time: Time::from_microseconds(write_time * 10.0),
        };

        future::ok(counters)
    })

}

pub fn io_counters() -> impl Stream<Item=Result<IoCounters>> {
    inner_stream(|_| true)
}

pub fn io_counters_physical() -> impl Stream<Item=Result<IoCounters>> {
    inner_stream(|path: &PathBuf| {
        disks::is_fixed_drive(path.as_path())
    })
}
