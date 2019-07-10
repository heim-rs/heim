use heim_common::prelude::*;
use heim_common::utils::fs;

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
        .try_fold(0, |acc, _| future::ok(acc + 1))
}

pub fn logical_count() -> impl Future<Output = Result<u64>> {
    sysconf().or_else(|_| cpuinfo())
    // TODO: Parse the `/proc/stat` to support old systems
    // See https://github.com/giampaolo/psutil/issues/200
}
