use std::marker::Unpin;
use std::path::Path;

use heim_common::prelude::*;
use heim_runtime::fs;

fn sysconf() -> Result2<u64> {
    let result = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };

    if result < 0 {
        Err(Error2::last_os_error().with_syscall(libc::_SC_NPROCESSORS_ONLN))
    } else {
        Ok(result as u64)
    }
}

async fn cpuinfo<T>(path: T) -> Result2<u64>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let mut result = 0;
    let mut lines = fs::read_lines(path);
    while let Some(line) = lines.next().await {
        if line?.starts_with("processor") {
            result += 1;
        }
    }

    Ok(result)
}

async fn stat<T>(path: T) -> Result2<u64>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let mut result = 0;
    let mut lines = fs::read_lines(path);
    // the first "cpu" line aggregates the numbers in all
    // of the other "cpuN" lines, hence skip the first item
    let mut first_skipped = false;

    while let Some(line) = lines.next().await {
        if line?.starts_with("cpu") {
            if first_skipped {
                result += 1;
            } else {
                first_skipped = true;
            }
        }
    }

    Ok(result)
}

pub async fn logical_count() -> Result2<u64> {
    if let result @ Ok(..) = sysconf() {
        return result;
    }

    if let result @ Ok(..) = cpuinfo("/proc/cpuinfo").await {
        return result;
    }

    stat("/proc/stat").await
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::{cpuinfo, stat};

    #[heim_derive::test]
    async fn test_stat_parse() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(PROC_STAT.as_bytes()).unwrap();

        let result = stat(f).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[heim_derive::test]
    async fn test_cpuinfo_parse() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(PROC_CPUINFO.as_bytes()).unwrap();

        let result = cpuinfo(f).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    const PROC_STAT: &str = include_str!("../../../../assets/linux_proc_stat.txt");
    const PROC_CPUINFO: &str = include_str!("../../../../assets/linux_proc_cpuinfo.txt");
}
