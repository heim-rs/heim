use std::str::FromStr;
use heim_common::prelude::*;
use heim_runtime as rt;
use crate::{Pid, Gid, Uid, Umask, ProcessResult, Status};
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
/*pub struct Groups {
    pub groups: Vec<Gid>,
}*/

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

impl Default for Status2 {
    fn default() -> Self {
        Self {
            name: String::default(),
            umask: Umask::default(),
            state: Status::Dead,
            tgid: Gid::default(),
            ngid: Gid::default(),
            pid: Pid::default(),
            ppid: Pid::default(),
            tracerpid: Pid::default(),
            uid: Uids::default(),
            gid: Gids::default(), 
        }
    }
    
}

impl FromStr for Status2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut status = Status2::default();

        let mut split_str = s.split('\n');
        while let Some(s) = split_str.next() {
            let mut ss = s.splitn(2, '\t');
            match ss.try_next()?.trim_end_matches(':') {
                "Name" => status.name = ss.try_parse_next()?,
                "Umask" => status.umask = ss.try_parse_next()?,
                "State" => status.state = ss.try_parse_next()?,
                "Tgid" => status.tgid = ss.try_parse_next()?,
                "Ngid" => status.ngid = ss.try_parse_next()?,
                "Pid" => status.pid = ss.try_parse_next()?,
                "PPid" => status.ppid = ss.try_parse_next()?,
                "TracerPid" => status.tracerpid = ss.try_parse_next()?,
                "Uid" => {
                    let mut uids = ss.try_next()?.split('\t');
                    status.uid.real = uids.try_parse_next()?;
                    status.uid.effective = uids.try_parse_next()?;
                    status.uid.saved = uids.try_parse_next()?;
                    status.uid.filesystem = uids.try_parse_next()?;
                }
                "Gid" => {
                    let mut gids = ss.try_next()?.split('\t');//.collect::<Vec<&str>>();
                    //let mut gids_iter = gids.iter();
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

pub async fn statuss(pid: Pid) -> ProcessResult<Status2> {
    rt::fs::read_into::<_, _, Error>(format!("/proc/{}/status", pid))
        .await
        .map_err(Into::into)
}
