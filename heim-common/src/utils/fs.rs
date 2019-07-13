//! Filesystem I/O abstractions.
//!
//! This module should be used only in `heim` sub-crates,
//! do not use it directly.
//!
//! ## Why in a hell there is a sync FS operations in the async crate?!
//!
//! At the moment `runtime` crate does not provides the abstractions for the files IO.
//! Yet, in our case, FS opts are needed only for Linux and only `procfs` is used there.
//! Since `procfs` stores data in the memory, it would not be very terrible to read this
//! data synchronously -- it still will be quick enough.
//!
//! When `runtime` crate will provide fully async FS abstractions,
//! we will switch to them.

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use crate::prelude::*;

/// Returns future which checks if path `T` points to some file.
pub fn path_exists<T>(path: T) -> impl Future<Output = bool>
where
    T: AsRef<Path> + Send + 'static,
{
    // TODO: It is using a "sync" API
    // but since it is used only at Linux and only for in-memory located
    // filesystems, it should not be a big problem, since that kind of IO
    // will not block so much.
    //
    // In any way, this thing should be refactored when `runtime` crate
    // will have the proper async FS I/O support.
    future::ready(path.as_ref().exists())
}

/// Read `path` file asynchronously and convert it contents into a string.
pub fn read_to_string<T>(path: T) -> impl Future<Output = Result<String>>
where
    T: AsRef<Path> + Send + 'static,
{
    // TODO: It is using a "sync" API
    // but since it is used only at Linux and only for in-memory located
    // filesystems, it should not be a big problem, since that kind of IO
    // will not block so much.
    //
    // In any way, this thing should be refactored when `runtime` crate
    // will have the proper async FS I/O support.
    let res = fs::read_to_string(path).map_err(From::from);

    future::ready(res)
}

// TODO: Probably should be renamed into `try_read_into`
/// Reads file and attempts to parse it's contents into `R` type.
pub fn read_into<T, R, E>(path: T) -> impl Future<Output = Result<R>>
where
    T: AsRef<Path> + Send + 'static,
    // TODO: Use `try_from::TryFrom` here too
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_to_string(path).then(|try_content| match try_content {
        Ok(content) => future::ready(R::from_str(&content).map_err(Error::from)),
        Err(e) => future::err(e),
    })
}

/// Returns stream of lines yielded from file with `path` path.
pub fn read_lines<T>(path: T) -> impl TryStream<Ok = String, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
    future::ready(fs::File::open(path))
        .map_err(Error::from)
        .map_ok(|file| {
            // TODO: It is using a "sync" API
            // but since it is used only at Linux and only for in-memory located
            // filesystems, it should not be a big problem, since that kind of IO
            // will not block so much.
            //
            // In any way, this thing should be refactored when `runtime` crate
            // will have the proper async FS I/O support.
            let reader = io::BufReader::new(file);
            stream::iter(reader.lines()).map_err(Error::from)
        })
        .try_flatten_stream()
}

/// Returns stream which reads lines from file and tries to parse them with help of `FromStr` trait.
pub fn read_lines_into<T, R, E>(path: T) -> impl TryStream<Ok = R, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr<Err = E>,
    Error: From<E>,
{
    read_lines(path).into_stream().then(|result| {
        let res = result.and_then(|line| R::from_str(&line).map_err(Error::from));

        future::ready(res)
    })
}

/// Returns future which tries to read the first line from file.
pub fn read_first_line<T>(path: T) -> impl TryFuture<Ok = String, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
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

/// Returns stream of files and directories contained in the `T` directory.
pub fn read_dir<T>(path: T) -> impl TryStream<Ok = fs::DirEntry, Error = Error>
where
    T: AsRef<Path> + Send + 'static,
{
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
