use std::fs;
use std::io::{self, BufRead};

use heim_common::prelude::*;
use heim_runtime as rt;

fn sysconf() -> Result<u64> {
    let result = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };

    if result < 0 {
        Err(Error::last_os_error().with_sysconf(libc::_SC_NPROCESSORS_ONLN))
    } else {
        Ok(result as u64)
    }
}

async fn cpuinfo() -> Result<u64> {
    rt::spawn_blocking(|| {
        let f = fs::File::open("/proc/cpuinfo")?;
        let reader = io::BufReader::new(f);
        let mut count = 0;
        for line in reader.lines() {
            if line?.starts_with("processor") {
                count += 1;
            }
        }

        Ok(count)
    })
    .await
}

async fn stat() -> Result<u64> {
    rt::spawn_blocking(|| {
        let f = fs::File::open("/proc/stat")?;
        let reader = io::BufReader::new(f);
        let mut count = 0;

        // the first "cpu" line aggregates the numbers in all
        // of the other "cpuN" lines, hence skip the first item
        for line in reader.lines().skip(1) {
            if line?.starts_with("cpu") {
                count += 1;
            }
        }

        Ok(count)
    })
    .await
}

pub async fn logical_count() -> Result<u64> {
    match sysconf() {
        Ok(value) => Ok(value),
        Err(..) => match cpuinfo().await {
            Ok(value) => Ok(value),
            Err(..) => stat().await,
        },
    }
}
