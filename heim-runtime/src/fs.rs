//! Async FS operations.

use std::io;
use std::path::Path;
use std::str::FromStr;

use futures_util::{future::ready, stream::Stream, TryStreamExt};

cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-async-std")] {
        pub use crate::shims::fs::*;
    }
}

/// Read `path` file and try to parse it into a `R` type via `std::str::FromStr`.
pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path>,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    let contents = read_to_string(path.as_ref()).await?;

    R::from_str(&contents)
}

/// Returns future which tries to read the first line from file.
pub async fn read_first_line_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path>,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    let line = read_first_line(path).await?;
    R::from_str(&line)
}

/// Returns stream which reads lines from file and tries to parse them with help of `FromStr` trait.
pub async fn read_lines_into<T, R, E>(path: T) -> io::Result<impl Stream<Item = Result<R, E>>>
where
    T: AsRef<Path>,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    let lines = read_lines(path).await?;
    let stream = lines
        .map_err(E::from)
        .and_then(|line| ready(R::from_str(&line)));
    Ok(stream)
}
