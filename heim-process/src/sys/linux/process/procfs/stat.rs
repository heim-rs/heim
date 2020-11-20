use std::io;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;
use heim_common::units::{time, Time};
use heim_common::utils::iter::{ParseIterator, TryIterator};
use heim_runtime as rt;

use crate::sys::linux::process::procfs::process_file_path;
use crate::{Pid, ProcessError, ProcessResult, Status};

impl Status {
    // Previously there were `TryFrom<char>` and `FromStr` implementations,
    // but they were leaking into the public API. See #260
    fn try_from_char(value: char) -> Result<Status> {
        match value {
            'R' => Ok(Status::Running),
            'S' => Ok(Status::Sleeping),
            'D' => Ok(Status::Waiting),
            'Z' => Ok(Status::Zombie),
            'T' => Ok(Status::Stopped),
            't' => Ok(Status::Tracing),
            'X' | 'x' => Ok(Status::Dead),
            'K' => Ok(Status::Wakekill),
            'W' => Ok(Status::Waking),
            'P' => Ok(Status::Parked),
            'I' => Ok(Status::Idle),
            other => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);

                Err(Error::from(inner).with_message(format!("Unknown process state {}", other)))
            }
        }
    }
}

#[derive(Debug)]
pub struct Stat {
    pub pid: Pid,
    pub name: String,
    pub state: Status,
    pub ppid: Pid,
    pub create_time: Time,
    pub utime: Time,
    pub stime: Time,
    pub cutime: Time,
    pub cstime: Time,
}

impl FromStr for Stat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ' ');
        let pid: Pid = parts.try_parse_next()?;
        let leftover = parts.try_next()?;
        let comm_end = leftover
            .rfind(')')
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        let name = leftover[1..comm_end].to_string();
        // `+ 2` is for the ") " at the start
        let mut parts = leftover[comm_end + 2..].split_whitespace();
        let state = parts.try_next().and_then(|str| {
            let chr = str
                .chars()
                .next()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
            Status::try_from_char(chr)
        })?;
        let ppid: Pid = parts.try_parse_next()?;
        let _pgrp: i32 = parts.try_parse_next()?;
        let _session_id: i32 = parts.try_parse_next()?;
        let _tty_nr: i32 = parts.try_parse_next()?;
        let _tpgid: i32 = parts.try_parse_next()?;
        let _flags: u32 = parts.try_parse_next()?;
        let _minflt: u64 = parts.try_parse_next()?;
        let _cminflt: u64 = parts.try_parse_next()?;
        let _majflt: u64 = parts.try_parse_next()?;
        let _cmajflt: u64 = parts.try_parse_next()?;
        let utime: u64 = parts.try_parse_next()?;
        let stime: u64 = parts.try_parse_next()?;
        let cutime: u64 = parts.try_parse_next()?;
        let cstime: u64 = parts.try_parse_next()?;
        let _priority: i64 = parts.try_parse_next()?;
        let _nice: i64 = parts.try_parse_next()?;
        let _num_threads: i64 = parts.try_parse_next()?;
        let _itrealvalue: i64 = parts.try_parse_next()?;
        let start_time: u64 = parts.try_parse_next()?;
        let _vsize: i64 = parts.try_parse_next()?;
        let _rss: i64 = parts.try_parse_next()?;
        let _rsslim: u64 = parts.try_parse_next()?;
        // ...

        // Note: we need to operate with `f64` in here for as much as possible,
        // otherwise we will lose the fraction part, leading to the same CPU time values
        // if called consequently. That breaks `top` example (and similar things),
        // so these fractions are really important here.
        let ticks = *CLOCK_TICKS as f64;

        // TODO: Potential precision loss during the `as f64` cast
        let start_time = start_time as f64 / ticks;
        debug_assert!(!start_time.is_nan());

        Ok(Stat {
            pid,
            name,
            state,
            ppid,
            create_time: Time::new::<time::second>(start_time),
            // TODO: Possible precision loss during the `as f64` cast
            utime: Time::new::<time::second>(utime as f64 / ticks),
            stime: Time::new::<time::second>(stime as f64 / ticks),
            cutime: Time::new::<time::second>(cutime as f64 / ticks),
            cstime: Time::new::<time::second>(cstime as f64 / ticks),
        })
    }
}

pub async fn stat(pid: Pid) -> ProcessResult<Stat> {
    let path = process_file_path(pid, "stat");
    // TODO: Get rid of the `.clone`
    let contents = match rt::fs::read_to_string(path.clone()).await {
        Ok(contents) => contents,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(ProcessError::NoSuchProcess(pid))
        }
        Err(e) => return Err(Error::from(e).with_file(path).into()),
    };

    let mut stats = Stat::from_str(&contents)?;
    let boot_time = heim_host::boot_time().await?;

    stats.create_time += boot_time;

    Ok(stats)
}
