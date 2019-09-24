use std::collections::HashSet;
use std::os::unix::ffi::OsStrExt;
use std::str;

use heim_common::prelude::*;
use heim_runtime::fs;

fn topology() -> impl Future<Output = Result<u64>> {
    let acc = HashSet::<u64>::new();
    fs::read_dir("/sys/devices/system/cpu/")
        .try_filter_map(|entry| {
            let matched = match entry.path().file_name() {
                Some(name) if name.as_bytes().starts_with(b"cpu") => {
                    // Safety: since it will be used with Linux only,
                    // it is okay to assume that /sys files will has the UTF-8 names
                    let core_id = unsafe { str::from_utf8_unchecked(&name.as_bytes()[3..]) };

                    match core_id.parse::<u64>() {
                        Ok(..) => Some(entry),
                        _ => None,
                    }
                }
                _ => None,
            };

            future::ok(matched)
        })
        .and_then(|entry| {
            let path = entry.path().join("topology/core_id");

            fs::read_to_string(path)
        })
        .map_err(Error::from)
        .and_then(|contents| future::ready(contents.trim().parse::<u64>().map_err(Error::from)))
        .try_fold(acc, |mut acc, cpu_id| {
            let _ = acc.insert(cpu_id);

            future::ok(acc)
        })
        .and_then(|acc| {
            if !acc.is_empty() {
                future::ok(acc.len() as u64)
            } else {
                // This error will not be propagated to caller,
                // since `physical_count` will call `or_else()` on it
                future::err(Error::incompatible("Unable to fetch CPU topology"))
            }
        })
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

fn cpu_info() -> impl Future<Output = Result<Option<u64>>> {
    let acc = Collector::default();

    fs::read_lines("/proc/cpuinfo")
        .map_err(Error::from)
        .try_fold(acc, |mut acc, line| {
            let result = match &line {
                l if l.starts_with("physical id") => match parse_line(l.as_str()) {
                    Ok(physical_id) if acc.physical_id.is_none() => {
                        acc.physical_id = Some(physical_id);

                        Ok(acc)
                    }
                    Ok(..) => {
                        panic!("Missed the core id value in the /proc/cpuinfo!");
                    }
                    Err(e) => Err(e),
                },
                l if l.starts_with("core id") => match parse_line(l.as_str()) {
                    Ok(core_id) if acc.physical_id.is_some() => {
                        let physical_id = acc
                            .physical_id
                            .take()
                            .expect("Will not happen, match guard covers that");
                        let _ = acc.group.insert((physical_id, core_id));

                        Ok(acc)
                    }
                    Ok(..) => {
                        panic!("Missed the physical id value in the /proc/cpuinfo!");
                    }
                    Err(e) => Err(e),
                },
                _ => Ok(acc),
            };

            future::ready(result)
        })
        .map_ok(|acc| {
            if !acc.group.is_empty() {
                Some(acc.group.len() as u64)
            } else {
                None
            }
        })
}

pub fn physical_count() -> impl Future<Output = Result<Option<u64>>> {
    topology().map_ok(Some).or_else(|_| cpu_info())
}
