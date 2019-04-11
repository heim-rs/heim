use std::path::{Path, PathBuf};

use heim_common::prelude::*;

use super::{pid_exists, pids};
use crate::Pid;
use crate::ProcessState;

mod procfs;
mod stat;

#[derive(Debug)]
pub struct Process {
    stat: stat::Stat,
    exe: Option<PathBuf>,
    //    env: EnvOs,
}

impl Process {
    pub fn from_pid(pid: Pid) -> impl Future<Item = Process, Error = Error> {
        Process::from_path(format!("/proc/{}/", pid))
    }

    /// Parse all needed information from the `/proc/{pid}/` folder
    pub fn from_path<T>(path: T) -> impl Future<Item = Process, Error = Error>
    where
        T: AsRef<Path> + Send + Clone + 'static,
    {
        // TODO: Get rid of the `path.clone()`
        let stat = stat::Stat::from_path(path.as_ref().join("stat"));
        let exe = procfs::read_exe(path.clone());
        //        let env_vars = procfs::read_environ(path);

        stat.join(exe).map(|(stat, exe)| Process {
            stat,
            exe,
        })
    }

    pub fn pid(&self) -> Pid {
        self.stat.pid()
    }

    pub fn ppid(&self) -> Pid {
        self.stat.ppid()
    }

    pub fn name(&self) -> &str {
        self.stat.name()
    }

    pub fn exe(&self) -> Option<&Path> {
        self.exe.as_ref().map(PathBuf::as_path)
    }

    pub fn state(&self) -> ProcessState {
        self.stat.state()
    }

    //    pub fn environ(&self) -> &EnvOs {
    //        &self.env
    //    }

    pub fn is_alive(&self) -> impl Future<Item = bool, Error = Error> {
        pid_exists(self.stat.pid())
    }
}

pub fn processes() -> impl Stream<Item = Process, Error = Error> {
    pids().and_then(Process::from_pid)
}
