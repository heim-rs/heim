use std::convert::TryFrom;
use std::io;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::sys::unix::CLOCK_TICKS;
use heim_common::units::{time, Time};
use heim_common::utils::iter::{ParseIterator, TryIterator};
use heim_runtime::fs;

use crate::{Pid, ProcessError, ProcessResult, Status};

impl TryFrom<char> for Status {
    type Error = Error;

    fn try_from(value: char) -> Result<Status> {
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
            other => Err(Error::incompatible(format!(
                "Unknown process state {}",
                other
            ))),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(value: &str) -> Result<Status> {
        match value.chars().next() {
            Some(chr) => Status::try_from(chr),
            // Can only mean a bug in implementation
            None => unreachable!(),
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
        let state: Status = parts.try_parse_next()?;
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
        let cutime: i64 = parts.try_parse_next()?;
        let cstime: i64 = parts.try_parse_next()?;
        let _priority: i64 = parts.try_parse_next()?;
        let _nice: i64 = parts.try_parse_next()?;
        let _num_threads: i64 = parts.try_parse_next()?;
        let _itrealvalue: i64 = parts.try_parse_next()?;
        let start_time: i64 = parts.try_parse_next()?;
        let _vsize: i64 = parts.try_parse_next()?;
        let _rss: i64 = parts.try_parse_next()?;
        let _rsslim: u64 = parts.try_parse_next()?;
        // ...

        let start_time = start_time as f64 / *CLOCK_TICKS;
        debug_assert!(!start_time.is_nan());

        Ok(Stat {
            pid,
            name,
            state,
            ppid,
            create_time: Time::new::<time::second>(start_time),
            // TODO: Possible values truncation during the `as f64` cast
            utime: Time::new::<time::second>(utime as f64 / *CLOCK_TICKS),
            stime: Time::new::<time::second>(stime as f64 / *CLOCK_TICKS),
            cutime: Time::new::<time::second>(cutime as f64 / *CLOCK_TICKS),
            cstime: Time::new::<time::second>(cstime as f64 / *CLOCK_TICKS),
        })
    }
}

pub fn stat(pid: Pid) -> impl Future<Output = ProcessResult<Stat>> {
    fs::read_to_string(format!("/proc/{}/stat", pid))
        .map_err(move |e| {
            if e.kind() == io::ErrorKind::NotFound {
                ProcessError::NoSuchProcess(pid)
            } else {
                e.into()
            }
        })
        .and_then(|contents| future::ready(Stat::from_str(&contents).map_err(Into::into)))
        .and_then(|mut stat| {
            heim_host::boot_time()
                .map_err(Into::into)
                .map_ok(move |boot_time| {
                    stat.create_time += boot_time;

                    stat
                })
        })
}
