use std::collections::HashSet;
use std::ffi::OsStr;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::str;

use heim_common::prelude::*;
use heim_runtime::fs;

// Check if file name matches the `cpu[0-9]+` pattern
fn match_cpu_n(file_name: Option<&OsStr>) -> bool {
    match file_name {
        // We want to match all `cpu[0-9]+` folders
        Some(name) if name.as_bytes().starts_with(b"cpu") && name.len() > 3 => {
            // Safety: since it will be used with Linux only,
            // it is okay to assume that /sys files will has the UTF-8 names.
            // In additional, we already had checked that `name` is larger than 3 bytes
            let core_id = unsafe { str::from_utf8_unchecked(&name.as_bytes()[3..]) };

            core_id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

async fn topology() -> Result<u64> {
    let mut acc = HashSet::<u64>::new();
    let mut cpu_entries = fs::read_dir("/sys/devices/system/cpu").await?;
    while let Some(entry) = cpu_entries.next().await {
        let entry = entry?;
        if !match_cpu_n(entry.path().file_name()) {
            continue;
        }

        let core_id_path = entry.path().join("topology/core_id");
        let contents = fs::read_to_string(core_id_path).await?;
        let core_id = contents.trim().parse::<u64>()?;
        let _ = acc.insert(core_id);
    }

    if !acc.is_empty() {
        Ok(acc.len() as u64)
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData).into())
    }
}

/// Parse value from the `/proc/cpuinfo` line.
///
/// Line is usually looks like `"physical id\t: 0"`
fn parse_line(line: &str) -> Result<u64> {
    line.split(':')
        .nth(1)
        .map(|value| value.trim())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData).into())
        .and_then(|value| value.parse::<u64>().map_err(Into::into))
}

/// What happens here: we are parsing the `/proc/cpuinfo` file line by line
/// and grouping consequent `"physical id: *"` and `"core id: *"` lines.
async fn cpu_info<T>(path: T) -> Result<Option<u64>>
where
    T: AsRef<Path>,
{
    let mut physical_id: Option<u64> = None;
    let mut group: HashSet<(u64, u64)> = HashSet::new();

    let mut lines = fs::read_lines(path).await?;
    while let Some(try_line) = lines.next().await {
        match &try_line? {
            line if line.starts_with("physical id") => {
                let core_id = parse_line(line)?;
                if physical_id.is_none() {
                    physical_id = Some(core_id);
                } else {
                    // TODO: Attach context data for error
                    return Err(io::Error::from(io::ErrorKind::InvalidData).into());
                }
            }
            line if line.starts_with("core id") => {
                let core_id = parse_line(line)?;
                if physical_id.is_some() {
                    let phys_id = physical_id
                        .take()
                        .expect("Unreachable, match guard covers this");
                    let _ = group.insert((phys_id, core_id));
                } else {
                    // TODO: Attach context data for error
                    return Err(io::Error::from(io::ErrorKind::InvalidData).into());
                }
            }
            _ => continue,
        }
    }

    if !group.is_empty() {
        Ok(Some(group.len() as u64))
    } else {
        Ok(None)
    }
}

pub async fn physical_count() -> Result<Option<u64>> {
    match topology().await {
        Ok(value) => Ok(Some(value)),
        Err(..) => cpu_info("/proc/cpuinfo").await,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::cpu_info;

    #[heim_derive::test]
    async fn test_cpuinfo_parse() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(PROC_CPUINFO.as_bytes()).unwrap();

        let result = cpu_info(f).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(2));
    }

    const PROC_CPUINFO: &str = include_str!("../../../../assets/linux_proc_cpuinfo.txt");
}
