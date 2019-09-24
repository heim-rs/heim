use std::ffi::{OsStr, OsString};

use heim_common::prelude::*;

use crate::{Pid, ProcessResult};

#[derive(Debug)]
pub struct Command;

impl Command {
    pub fn to_os_string(&self) -> OsString {
        unimplemented!()
    }

    pub fn into_os_string(self) -> OsString {
        unimplemented!()
    }
}

impl<'a> IntoIterator for &'a Command {
    type Item = &'a OsStr;
    type IntoIter = CommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CommandIter {
            _marker: ::std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct CommandIter<'a> {
    _marker: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Iterator for CommandIter<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub fn command(_pid: Pid) -> impl Future<Output = ProcessResult<Command>> {
    future::ok(Command {})
}
