use std::fmt;
use std::path::Path;

use heim_common::prelude::*;

use crate::{sys, Pid, ProcessState};
use crate::types::EnvOs;

#[derive(heim_derive::ImplWrap)]
pub struct Process(sys::Process);

impl Process {
    pub fn from_pid(pid: Pid) -> impl Future<Item=Process, Error=Error> {
        sys::Process::from_pid(pid).map(Into::into)
    }

    /// Returns process PID.
    pub fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    /// Returns process parent PID.
    pub fn parent_pid(&self) -> Pid {
        self.as_ref().ppid()
    }

    /// Returns process name.
    pub fn name(&self) -> &str {
        self.as_ref().name()
    }

    pub fn exe(&self) -> Option<&Path> {
        self.as_ref().exe()
    }

    /// Returns process state.
    pub fn state(&self) -> ProcessState {
        self.as_ref().state()
    }

//    /// Returns process environment
//    // TODO: Move to `ProcessExt`
//    #[doc(hidden)]
//    pub fn environ(&self) -> &EnvOs {
//        self.as_ref().environ()
//    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Process")
            .field("pid", &self.pid())
            .field("name", &self.name())
            .field("state", &self.state())
            .field("parent_pid", &self.parent_pid())
            .field("exe", &self.exe())
            .finish()
    }
}

pub fn processes() -> impl Stream<Item=Process, Error=Error> {
    sys::processes().map(Into::into)
}
