use std::io;
use std::path::Path;
use std::str::FromStr;

use tokio::prelude::*;

use crate::Error;

pub fn path_exists<T>(path: T) -> impl Future<Item=bool, Error=Error>
where
    T: AsRef<Path> + Send + 'static
{
    tokio::fs::metadata(path)
        .map(|_| true)
        .map_err(Error::from)
}

/// Read `path` file asynchronously and convert it contents into a string.
pub fn read_to_string<T>(path: T) -> impl Future<Item = String, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
    tokio::fs::read(path)
        .map_err(Error::from)
        .and_then(|bytes| Ok(String::from_utf8(bytes)?))
}

pub fn read_into<T, R, E>(path: T) -> impl Future<Item = R, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
    // TODO: Use `try_from::TryFrom` here too
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_to_string(path).and_then(|string| Ok(R::from_str(&string)?))
}

/// Returns stream of lines yielded from file with `path` path.
pub fn read_lines<T>(path: T) -> impl Stream<Item = String, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
    tokio::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map(|file| {
            let reader = io::BufReader::new(file);
            tokio::io::lines(reader)
        })
        .flatten_stream()
        .map_err(Error::from)
}

pub fn read_lines_into<T, R, E>(path: T) -> impl Stream<Item = R, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_lines(path).and_then(|line| Ok(R::from_str(&line)?))
}
