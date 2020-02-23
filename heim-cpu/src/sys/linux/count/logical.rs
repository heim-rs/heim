use heim_common::prelude::*;
use heim_runtime as rt;

fn sysconf() -> Result<u64> {
    let result = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };

    if result < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(result as u64)
    }
}

async fn cpuinfo() -> Result<u64> {
    let mut lines = rt::fs::read_lines("/proc/cpuinfo").await?;
    let mut count = 0;
    while let Some(line) = lines.next().await {
        let line = line?;
        if line.starts_with("processor") {
            count += 1;
        }
    }

    Ok(count)
}

async fn stat() -> Result<u64> {
    // the first "cpu" line aggregates the numbers in all
    // of the other "cpuN" lines, hence skip the first item
    let mut lines = rt::fs::read_lines("/proc/stat").await?.skip(1);

    let mut count = 0;
    while let Some(line) = lines.next().await {
        let line = line?;
        if line.starts_with("cpu") {
            count += 1;
        }
    }

    Ok(count)
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
