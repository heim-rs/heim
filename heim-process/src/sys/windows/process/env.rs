use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Environment;

impl Environment {
    pub fn iter(&self) -> EnvironmentIter {
        unimplemented!()
    }
}

impl IntoIterator for Environment {
    type Item = (OsString, OsString);
    type IntoIter = IntoEnvironmentIter;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

impl<'e> IntoIterator for &'e Environment {
    type Item = (&'e OsStr, &'e OsStr);
    type IntoIter = EnvironmentIter<'e>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct IntoEnvironmentIter;

impl Iterator for IntoEnvironmentIter {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EnvironmentIter<'e>(&'e PhantomData<()>);

impl<'e> Iterator for EnvironmentIter<'e> {
    type Item = (&'e OsStr, &'e OsStr);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}
