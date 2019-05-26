use std::path::Path;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::iter::*;

use crate::{Pid, ProcessState};

/// Parsed contents of the `/proc/{pid}/stat` file.
///
/// See `proc(5)` for format details.
#[derive(Debug)]
pub struct Stat {
    pid: Pid,
    name: String,
    state: ProcessState,
    ppid: Pid,
}

impl FromStr for Stat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Stat> {
        let mut parts = s.splitn(2, ' ');
        let pid: Pid = parts.try_from_next()?;
        let rest = parts.next().ok_or_else(|| Error::new(ErrorKind::Parse))?;
        let name_end = rest
            .chars()
            .enumerate()
            .skip(1)
            .position(|(_, c)| c == ')')
            .ok_or_else(|| Error::new(ErrorKind::Parse))?;
        let name = rest[1..=name_end].to_string();
        // Skipping the ") " part.
        let mut parts = rest[name_end + 3..].split_whitespace();
        let state: ProcessState = parts.try_from_next()?;
        let ppid: Pid = parts.try_from_next()?;

        Ok(Stat {
            pid,
            name,
            state,
            ppid,
        })
    }
}

impl Stat {
    pub fn from_path<T>(path: T) -> impl Future<Item = Stat, Error = Error>
    where
        T: AsRef<Path> + Send + 'static,
    {
        utils::fs::read_into(path)
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn state(&self) -> ProcessState {
        self.state
    }

    pub fn ppid(&self) -> Pid {
        self.ppid
    }
}

impl FromStr for ProcessState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // TODO: Check if comparing to first char is faster
        let res = match s {
            "R" => ProcessState::Running,
            "S" => ProcessState::Sleeping,
            "D" => ProcessState::DiskSleep,
            "Z" => ProcessState::Zombie,
            "T" => ProcessState::Stopped,
            "t" => ProcessState::TracingStop,
            "X" | "x" => ProcessState::Dead,
            "K" => ProcessState::WakeKill,
            "W" => ProcessState::Waking,
            "P" => ProcessState::Parked,
            "I" => ProcessState::Idle,
            other => unreachable!("Unknown process state {}", other),
        };

        Ok(res)
    }
}
