use std::collections::HashSet;
use std::os::unix::ffi::OsStrExt;
use std::str;

use heim_common::prelude::*;
use heim_runtime as rt;

async fn topology() -> Result<u64> {
    let mut acc = HashSet::<u64>::new();
    let mut entries = rt::fs::read_dir("/sys/devices/system/cpu/").await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;

        // TODO: Whole block looks ugly, rewrite it.
        // What it does: checks if entry name conforms to `cpu\d+` pattern.
        match entry.path().file_name() {
            Some(name) if name.as_bytes().starts_with(b"cpu") => {
                // Safety: since it will be used with Linux only,
                // for it is okay to assume that /sys files will has the UTF-8 names.
                // TODO: Make it safe instead.
                let core_id = unsafe { str::from_utf8_unchecked(&name.as_bytes()[3..]) };

                match core_id.parse::<u64>() {
                    Ok(..) => {}
                    _ => continue,
                }
            }
            _ => continue,
        };

        let path = entry.path().join("topology/core_id");
        let contents = rt::fs::read_to_string(path).await?;
        let cpu_id = contents.trim().parse()?;

        let _ = acc.insert(cpu_id);
    }

    if !acc.is_empty() {
        Ok(acc.len() as u64)
    } else {
        // This error will not be propagated to caller,
        // since `physical_count` will call `or_else()` on it
        Err(Error::incompatible("Unable to fetch CPU topology"))
    }
}

#[derive(Default)]
struct Collector {
    physical_id: Option<u64>,
    group: HashSet<(u64, u64)>,
}

fn parse_line(line: &str) -> Result<u64> {
    line.split(':')
        .nth(2)
        .map(|value| value.trim())
        .ok_or_else(|| Error::incompatible("Unsupported format for /proc/cpuinfo"))
        .and_then(|value| value.parse::<u64>().map_err(Error::from))
}

async fn cpu_info() -> Result<Option<u64>> {
    let mut acc = Collector::default();

    let mut lines = rt::fs::read_lines("/proc/cpuinfo").await?;
    while let Some(line) = lines.next().await {
        match &line? {
            l if l.starts_with("physical id") => {
                let core_id = parse_line(l.as_str())?;
                if acc.physical_id.is_none() {
                    acc.physical_id = Some(core_id)
                } else {
                    // TODO: In general it seems better to return an error
                    panic!("Missed the core id value in the /proc/cpuinfo, implementation bug");
                }
            }
            l if l.starts_with("core id") => {
                let core_id = parse_line(l.as_str())?;
                if acc.physical_id.is_some() {
                    let physical_id = acc.physical_id.take().expect("Not expected to be happen");
                    let _ = acc.group.insert((physical_id, core_id));
                } else {
                    // TODO: In general it seems better to return an error
                    panic!("Missed the physical id value in the /proc/cpuinfo!");
                }
            }
            _ => continue,
        }
    }

    if !acc.group.is_empty() {
        Ok(Some(acc.group.len() as u64))
    } else {
        Ok(None)
    }
}

pub async fn physical_count() -> Result<Option<u64>> {
    match topology().await {
        Ok(count) => Ok(Some(count)),
        Err(..) => cpu_info().await,
    }
}
