use std::str::FromStr;
use heim_common::prelude::*;
use heim_runtime as rt;
use crate::{Pid, Gid, Uid, Umask, ProcessResult, Status as State};
use heim_common::utils::iter::{ParseIterator, TryIterator};

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

pub struct Status {
    pub name: String,
    pub umask: Umask,
    pub state: State,
    pub tgid: Gid,
    pub ngid: Gid,
    pub pid: Pid,
    pub ppid: Pid,
    pub tracer_pid: Pid,
    pub uid: Uids,
    pub gid: Gids,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            name: String::default(),
            umask: Umask::default(),
            state: State::Running,
            tgid: Gid::default(),
            ngid: Gid::default(),
            pid: Pid::default(),
            ppid: Pid::default(),
            tracer_pid: Pid::default(),
            uid: Uids::default(),
            gid: Gids::default(),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut status = Status::default();

        let mut split_str = s.split('\n');
        while let Some(s) = split_str.next() {
            let mut col = s.splitn(2, '\t');
            match col.try_next()?.trim_end_matches(':') {
                "Name" => status.name = col.try_parse_next()?,
                "Umask" => status.umask = col.try_parse_next()?,
                "State" => status.state = col.try_parse_next()?,
                "Tgid" => status.tgid = col.try_parse_next()?,
                "Ngid" => status.ngid = col.try_parse_next()?,
                "Pid" => status.pid = col.try_parse_next()?,
                "PPid" => status.ppid = col.try_parse_next()?,
                "TracerPid" => status.tracer_pid = col.try_parse_next()?,
                "Uid" => {
                    let mut uids = col.try_next()?.split('\t');
                    status.uid.real = uids.try_parse_next()?;
                    status.uid.effective = uids.try_parse_next()?;
                    status.uid.saved = uids.try_parse_next()?;
                    status.uid.filesystem = uids.try_parse_next()?;
                }
                "Gid" => {
                    let mut gids = col.try_next()?.split('\t');
                    status.gid.real = gids.try_parse_next()?;
                    status.gid.effective = gids.try_parse_next()?;
                    status.gid.saved = gids.try_parse_next()?;
                    status.gid.filesystem = gids.try_parse_next()?;
                }
                 _ => {
                     break;
                 }
             }
        }
        Ok(status)
    }
}

pub async fn status(pid: Pid) -> ProcessResult<Status> {
    rt::fs::read_into::<_, _, Error>(format!("/proc/{}/status", pid))
        .await
        .map_err(Into::into)
}
