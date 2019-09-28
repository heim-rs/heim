use heim_common::prelude::*;
use heim_runtime::fs;

fn sysconf() -> impl Future<Output = Result<u64>> {
    let result = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };

    if result < 0 {
        future::err(Error::last_os_error())
    } else {
        future::ok(result as u64)
    }
}

fn cpuinfo() -> impl Future<Output = Result<u64>> {
    fs::read_lines("/proc/cpuinfo")
        .try_filter(|line| future::ready(line.starts_with("processor")))
        .map_err(Error::from)
        .try_fold(0, |acc, _| future::ok(acc + 1))
}

fn stat() -> impl Future<Output = Result<u64>> {
    fs::read_lines("/proc/stat")
        .try_filter(|line| future::ready(line.starts_with("cpu")))
        .map_err(Error::from)
        // the first "cpu" line aggregates the numbers in all
        // of the other "cpuN" lines, hence skip the first item
        .skip(1)
        .try_fold(0, |acc, _| future::ok(acc + 1))
}

pub fn logical_count() -> impl Future<Output = Result<u64>> {
    sysconf().or_else(|_| cpuinfo()).or_else(|_| stat())
}
