//use std::convert::TryFrom;
//use std::io;
use std::str::FromStr;

use heim_common::prelude::*;
//use heim_common::units::{time, Time};
//use heim_common::utils::iter::{ParseIterator, TryIterator};
use heim_runtime as rt;
use crate::{Pid, Gid, Uid, Umask, ProcessResult, Status};
/*
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
            other => {
                let inner = io::Error::from(io::ErrorKind::InvalidData);

                Err(Error::from(inner).with_message(format!("Unknown process state {}", other)))
            }
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
}*/
#[derive(Default)]
pub struct Uids {
    pub real: Uid,
    pub effective: Uid,
    pub saved: Uid,
    pub filesystem: Uid
}
#[derive(Default)]
pub struct Gids {
    pub real: Gid,
    pub effective: Gid,
    pub saved: Gid,
    pub filesystem: Gid
}
/*pub struct Groups {
    pub groups: Vec<Gid>,
}*/
//#[derive(Debug)]
pub struct Status2 {
    pub name: String,
    pub umask: Umask,
    pub state: Status,
    pub tgid: Gid,
    pub ngid: Gid,
    pub pid: Pid,
    pub ppid: Pid,
    pub tracerpid: Pid,
    pub uid: Uids,
    pub gid: Gids,
}

impl FromStr for Status2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.splitn(2, '\n');
        println!("{:?}", parts);
        /*
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


        let start_time = start_time as f64 / *CLOCK_TICKS;
        debug_assert!(!start_time.is_nan())*/

        Ok(Status2 {
            name: "AA".to_string(),
            umask: 0,
            state: Status::Running,
            tgid: 0,
            ngid: 0,
            pid: 0,
            ppid: 0,
            tracerpid: 0,
            uid: Uids::default(),
            gid: Gids::default(),
        })
    }
}

pub async fn statuss(pid: Pid) -> ProcessResult<Status2> {
    rt::fs::read_into::<_, _, Error>(format!("/proc/{}/status", pid))
        .await
        .map_err(Into::into)
    /*let path = format!("/proc/{}/status", pid);
    let contents = match rt::fs::read_to_string(&path).await {
        Ok(contents) => contents,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(ProcessError::NoSuchProcess(pid))
        }
        Err(e) => return Err(Error::from(e).with_file(path).into()),
    };

    let mut stats = Status2::from_str(&contents)?;*/

    //Ok(stats)
}
