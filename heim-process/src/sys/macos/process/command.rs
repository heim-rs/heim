use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::ffi::OsStrExt;

use crate::sys::macos::{pid_exists, wrappers};
use crate::{Pid, ProcessError, ProcessResult};

#[derive(Debug)]
pub struct Command(wrappers::ProcArgs);

impl Command {
    pub fn to_os_string(&self) -> OsString {
        self.0.to_command()
    }

    pub fn into_os_string(self) -> OsString {
        // TODO: Performance could be better
        self.to_os_string()
    }
}

impl<'a> IntoIterator for &'a Command {
    type Item = &'a OsStr;
    type IntoIter = CommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CommandIter(self.0.arguments())
    }
}

#[derive(Debug)]
pub struct CommandIter<'a>(wrappers::ProcArgsArguments<'a>);

impl<'a> Iterator for CommandIter<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(OsStr::from_bytes)
    }
}

pub async fn command(pid: Pid) -> ProcessResult<Command> {
    match wrappers::ProcArgs::get(pid) {
        Ok(proc_args) => Ok(Command(proc_args)),
        Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
            if pid_exists(pid).await? {
                Err(ProcessError::ZombieProcess(pid))
            } else {
                Err(e.into())
            }
        }
        Err(e) if e.as_inner().kind() == io::ErrorKind::PermissionDenied => {
            Err(ProcessError::AccessDenied(pid))
        }
        Err(e) => Err(e.into()),
    }
}
