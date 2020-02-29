use std::ffi::{OsStr, OsString};
use std::fmt;

use crate::sys;
use heim_common::prelude::wrap;

/// Process environment variables.
///
/// This structure is created by [`Process::environment`] method.
/// See its documentation for more.
///
/// [`Process::environment`]: ./struct.Process.html#method.environment
pub struct Environment(sys::Environment);

wrap!(Environment, sys::Environment);

impl Environment {
    /// Returns a non-consuming iterator over environment variables.
    pub fn iter(&self) -> EnvironmentIter<'_> {
        EnvironmentIter::from(self.0.iter())
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl IntoIterator for Environment {
    type Item = (OsString, OsString);
    type IntoIter = IntoEnvironmentIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoEnvironmentIter::from(self.0.into_iter())
    }
}

impl<'e> IntoIterator for &'e Environment {
    type Item = (&'e OsStr, &'e OsStr);
    type IntoIter = EnvironmentIter<'e>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator that moves out from the process environment variables.
///
/// This struct is created by the `into_iter` method on [`Environment`].
///
/// [`Environment`]: ./struct.Environment.html
#[derive(Debug)]
pub struct IntoEnvironmentIter(sys::IntoEnvironmentIter);

wrap!(IntoEnvironmentIter, sys::IntoEnvironmentIter);

impl Iterator for IntoEnvironmentIter {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// A non-consuming iterator for the process environment variables.
///
/// This struct is created by the `iter` method on [`Environment`].
///
/// [`Environment`]: ./struct.Environment.html
#[derive(Debug)]
pub struct EnvironmentIter<'e>(sys::EnvironmentIter<'e>);

wrap!('e, EnvironmentIter<'e>, sys::EnvironmentIter<'e>);

impl<'e> Iterator for EnvironmentIter<'e> {
    type Item = (&'e OsStr, &'e OsStr);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
