//! Async FS operations.

use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use futures_util::{future::ready, stream::Stream, TryStreamExt};

use crate::shims;

mod dir_entry;
mod file;

pub use self::dir_entry::DirEntry;
pub use self::file::File;

/// Returns future which checks if path `path` points to some file.
pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    shims::fs::path_exists(path).await
}

/// Read `path` file asynchronously and convert it contents into a string.
pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    shims::fs::read_to_string(path).await
}

/// Returns stream of lines yielded from file with `path` path.
pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    shims::fs::read_lines(path).await
}

/// Returns future which tries to read the first line from file.
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    shims::fs::read_first_line(path).await
}

/// Returns future which tries read the symlink.
pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path>,
{
    shims::fs::read_link(path).await
}

/// Returns stream of files and directories contained in the `path` directory.
pub async fn read_dir<T>(path: T) -> io::Result<impl Stream<Item = io::Result<DirEntry>>>
where
    T: AsRef<Path>,
{
    let entries = shims::fs::read_dir(path).await?;

    Ok(entries.map_ok(DirEntry::from))
}

/// Read `path` file and try to parse it into a `R` type via `std::str::FromStr`.
pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path>,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    let contents = read_to_string(path).await?;

    R::from_str(&contents)
}

/// Returns future which tries to read the first line from file.
pub async fn read_first_line_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path>,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    let line = shims::fs::read_first_line(path).await?;
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
