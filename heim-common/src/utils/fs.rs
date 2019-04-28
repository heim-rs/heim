// Why in a hell there is a sync FS operations in the async crate?!
//
// At the moment `runtime` crate does not provides the abstractions for the files IO.
// Yet, in our case, FS opts are needed only for Linux and only `procfs` is used there.
// Since `procfs` stores data in the memory, it would not be very terrible to read this
// data synchronously -- it still will be quick enough.
//
// When `runtime` crate will provide fully async FS abstractions,
// we will switch to them.

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::pin::Pin;
use std::str::FromStr;

use crate::prelude::*;

pub fn path_exists<T>(path: T) -> impl Future<Output = bool>
where
    T: AsRef<Path> + Send + 'static,
{
    future::ready(path.as_ref().exists())
}

/// Read `path` file asynchronously and convert it contents into a string.
pub fn read_to_string<T>(path: T) -> impl Future<Output = Result<String>>
where
    T: AsRef<Path> + Send + 'static,
{
    let res = fs::read_to_string(path).map_err(From::from);

    future::ready(res)
}

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
    // https://github.com/rust-lang-nursery/futures-rs/issues/1444
    future::ready(fs::File::open(path))
        .map_err(Error::from)
        .map_ok(|file| {
            let reader = io::BufReader::new(file);
            let stream = stream::iter(reader.lines()).map_err(Error::from);

            Box::pin(stream) as Pin<Box<dyn Stream<Item = _> + Send>>
        })
        .unwrap_or_else(|e| Box::pin(stream::once(future::err(e))))
        .flatten_stream()
}

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
