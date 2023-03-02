use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::str;

use heim_common::prelude::*;
use heim_runtime as rt;

async fn topology() -> Result<u64> {
    rt::spawn_blocking(|| {
        let path = rt::linux::sysfs_root().join("devices/system/cpu/cpu*/topology/core_id");
        let entries =
            glob::glob(path.display().to_string().as_str()).expect("Invalid glob pattern");
        let mut acc = HashSet::<u64>::new();

        for entry in entries {
            let entry = entry.map_err(|e| e.into_error())?;
            let contents = fs::read_to_string(entry)?;

            let core_id = contents.trim().parse()?;
            let _ = acc.insert(core_id);
        }

        if !acc.is_empty() {
            Ok(acc.len() as u64)
        } else {
            // This error will not be propagated to caller,
            // since `physical_count` will call `or_else()` on it
            Err(Error::from(io::Error::from(io::ErrorKind::InvalidData)))
        }
    })
    .await
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
        .ok_or_else(|| Error::from(io::Error::from(io::ErrorKind::InvalidData)))
        .and_then(|value| value.parse::<u64>().map_err(Error::from))
}

async fn cpu_info() -> Result<Option<u64>> {
    rt::spawn_blocking(|| {
        let mut acc = Collector::default();
        let f = fs::File::open(rt::linux::procfs_root().join("cpuinfo"))?;
        let reader = io::BufReader::new(f);

        let lines = reader.lines();
        for line in lines {
            match &line? {
                l if l.starts_with("physical id") => {
                    let core_id = parse_line(l.as_str())?;
                    if acc.physical_id.is_none() {
                        acc.physical_id = Some(core_id)
                    } else {
                        // TODO: In general it seems better to return an error
                        panic!(
                            "Missed the core id value in the {:?}/cpuinfo, implementation bug",
                            rt::linux::procfs_root()
                        );
                    }
                }
                l if l.starts_with("core id") => {
                    let core_id = parse_line(l.as_str())?;
                    if acc.physical_id.is_some() {
                        let physical_id =
                            acc.physical_id.take().expect("Not expected to be happen");
                        let _ = acc.group.insert((physical_id, core_id));
                    } else {
                        // TODO: In general it seems better to return an error
                        panic!(
                            "Missed the physical id value in the {:?}/cpuinfo!",
                            rt::linux::procfs_root()
                        );
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
    })
    .await
}

pub async fn physical_count() -> Result<Option<u64>> {
    match topology().await {
        Ok(count) => Ok(Some(count)),
        Err(..) => cpu_info().await,
    }
}
