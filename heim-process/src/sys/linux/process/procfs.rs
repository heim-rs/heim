use std::path::{Path, PathBuf};

use crate::EnvOs;
use heim_common::prelude::*;

pub fn read_exe<T: AsRef<Path>>(path: T) -> impl Future<Item = Option<PathBuf>, Error = Error> {
    let path = path.as_ref().join("exe");
    tokio::fs::read_link(path)
        .map(Some)
        // TODO: psutil also checks if ENOENT or ESRCH returned
        // https://github.com/giampaolo/psutil/blob/05d51649ca709c6626d84cc710c2470d64829848/psutil/_pslinux.py#L1624
        // At the moment returning `None` all the time
        .or_else(|_| Ok(None))
}

#[allow(dead_code)]
pub fn read_environ<T: AsRef<Path>>(path: T) -> impl Future<Item = EnvOs, Error = Error> {
    let path = path.as_ref().join("environ");
    tokio::fs::read(path)
        .map(|contents| EnvOs::from_bytes(&contents))
        .map_err(Error::from)
}
