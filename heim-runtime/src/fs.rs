use std::io;
use std::marker::Unpin;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use futures::stream::{Stream, StreamExt, TryStreamExt};

use super::runtime;
pub use runtime::fs::{path_exists, read_dir, read_lines, read_link, read_to_string};

/// Read `path` file as a string and try to convert it into `R`
pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path> + Send,
    R: FromStr,
    E: From<io::Error> + From<<R as FromStr>::Err>,
{
    let contents = read_to_string(path).await?;

    R::from_str(&contents).map_err(Into::into)
}

// TODO: Would be nice to remove `Box` eventually
/// Read `path` file and try to convert each line into `R`.
pub async fn read_lines_into<T, R, E>(path: T) -> io::Result<impl Stream<Item = Result<R, E>>>
where
    T: AsRef<Path> + Send,
    R: FromStr,
    E: From<io::Error> + From<<R as FromStr>::Err> + 'static,
{
    let lines = read_lines(path).await?;

    let parsed = lines
        .map_err(E::from)
        .and_then(|line| futures::future::ready(R::from_str(&line).map_err(E::from)));

    Ok(parsed)
}

/// Read first line from the `path` file
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send,
{
    let mut lines = read_lines(path).await?;
    match lines.next().await {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => Err(e),
        None => Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}
