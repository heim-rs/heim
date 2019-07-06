use std::error;
use std::fmt;

use heim_common::Error;

use crate::Pid;

#[derive(Debug)]
pub enum ProcessError {
    NoSuchProcess(Pid),
    ZombieProcess(Pid),
    Load(Error),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::NoSuchProcess(pid) => f.write_fmt(format_args!("Process {} does not exists", pid)),
            ProcessError::ZombieProcess(pid) => f.write_fmt(format_args!("Process {} is zombie", pid)),
            ProcessError::Load(e) => fmt::Display::fmt(e, f),
            _ => unreachable!(),
        }
    }
}

impl error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ProcessError::Load(e) => Some(e),
            _ => None,
        }
    }
}

impl From<Error> for ProcessError {
    fn from(e: Error) -> Self {
        ProcessError::Load(e)
    }
}
