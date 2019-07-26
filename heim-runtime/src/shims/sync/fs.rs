use std::fs;
use std::io::{self, BufRead as _};
use std::path::Path;
use std::str::FromStr;
#[cfg(target_os = "windows")]
use std::os::windows::io::{RawHandle, AsRawHandle};

use heim_common::prelude::*;

#[derive(Debug)]
pub struct File(fs::File);

impl File {
    pub fn open(path: &Path) -> impl Future<Output = Result<File>> {
        future::ready(fs::File::open(path))
            .map_ok(File)
            .map_err(Error::from)
    }

    #[cfg(target_os = "windows")]
    pub fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}

pub fn path_exists(path: &Path) -> impl Future<Output = bool> {
    future::ready(path.exists())
}

pub fn read_to_string(path: &Path) -> impl Future<Output = Result<String>> {
    future::ready(fs::read_to_string(path)).map_err(From::from)
}

pub fn read_into<R, E>(path: &Path) -> impl Future<Output = Result<R>>
where
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_to_string(path)
        .and_then(|content| {
            future::ready(R::from_str(&content).map_err(Error::from))
        })
}

pub fn read_lines(path: &Path) -> impl TryStream<Ok = String, Error = Error> {
    future::ready(fs::File::open(path))
        .map_err(Error::from)
        .map_ok(|file| {
            let reader = io::BufReader::new(file);
            stream::iter(reader.lines()).map_err(Error::from)
        })
        .try_flatten_stream()
}

pub fn read_lines_into<R, E>(path: &Path) -> impl TryStream<Ok = R, Error = Error>
where
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_lines(path).into_stream().then(|result| {
        let res = result.and_then(|line| R::from_str(&line).map_err(Error::from));

        future::ready(res)
    })
}

pub fn read_first_line(path: &Path) -> impl TryFuture<Ok = String, Error = Error> {
    // TODO: Looks dumb
    read_lines(path)
        .into_stream()
        .into_future()
        .map(|(try_line, _)| match try_line {
            Some(Ok(line)) => Ok(line),
            Some(Err(e)) => Err(e),
            None => Err(Error::missing_entity("line")),
        })
}

pub fn read_dir(path: &Path) -> impl TryStream<Ok = fs::DirEntry, Error = Error> {
    // TODO: It is using a "sync" API
    // but since it is used only at Linux and only for in-memory located
    // filesystems, it should not be a big problem, since that kind of IO
    // will not block so much.
    //
    // In any way, this thing should be refactored when `runtime` crate
    // will have the proper async FS I/O support.
    future::ready(fs::read_dir(path))
        .map_err(Error::from)
        .map_ok(|iter| stream::iter(iter).map_err(Error::from))
        .try_flatten_stream()
}
