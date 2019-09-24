use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use heim_common::prelude::*;

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

pub fn command(pid: Pid) -> impl Future<Output = ProcessResult<Command>> {
    future::lazy(move |_| wrappers::ProcArgs::get(pid))
        .map_ok(Command)
        .or_else(move |e| {
            // TODO: Will look better with `async_await`
            match e.raw_os_error() {
                // `KERN_PROCARGS2` syscall might return `EINVAL` in case of zombie process
                Some(libc::EINVAL) => {
                    let f = pid_exists(pid).and_then(move |is_exists| {
                        if is_exists {
                            future::err(ProcessError::ZombieProcess(pid))
                        } else {
                            future::err(e.into())
                        }
                    });

                    future::Either::Left(f)
                }
                _ => {
                    let f = future::err(e.into());

                    future::Either::Right(f)
                }
            }
        })
}
