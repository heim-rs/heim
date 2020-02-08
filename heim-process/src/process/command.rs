use std::ffi::{OsStr, OsString};
use std::fmt;

use crate::sys;
use heim_common::prelude::wrap;

/// Process command line.
pub struct Command(sys::Command);

wrap!(Command, sys::Command);

impl Command {
    /// Create an `OsString` containing the process command line.
    ///
    /// Spaces are used as a delimiters in the returned `OsString`.
    ///
    /// ## Notes
    ///
    /// This method will always allocate memory on all OSes.
    pub fn to_os_string(&self) -> OsString {
        self.as_ref().to_os_string()
    }

    /// Consumes `self` and returns the underline process command line.
    ///
    /// Spaces are used as a delimiters in the returned `OsString`.
    ///
    /// ## Notes
    ///
    /// This method might allocate on some OSes, depending on the implementation.
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Command")
            .field(&self.to_os_string())
            .finish()
    }
}

/// Iterator over process command line arguments.
#[derive(Debug)]
pub struct CommandIter<'a>(sys::CommandIter<'a>);

wrap!('a, CommandIter<'a>, sys::CommandIter<'a>);

impl<'a> IntoIterator for &'a Command {
    type Item = &'a OsStr;
    type IntoIter = CommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().into()
    }
}

impl<'a> Iterator for CommandIter<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
