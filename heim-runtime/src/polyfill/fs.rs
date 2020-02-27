use std::fs;
use std::io::{self, BufRead as _, BufReader};
use std::marker::Unpin;
use std::path::{Path, PathBuf};

use futures_core::Stream;
use futures_util::stream;

use super::blocking::spawn;

/// Asynchronously check if `path` exists.
pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path> + Send,
{
    spawn(move || path.as_ref().exists()).await
}

/// Asynchronously read the entire contents of a file into a string.
pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send,
{
    spawn(move || fs::read_to_string(path)).await
}

pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path> + Send,
{
    spawn(move || fs::read_link(path)).await
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>> + Unpin>
where
    T: AsRef<Path> + Send,
{
    spawn(move || {
        let file = fs::File::open(path)?;
        let lines = BufReader::new(file).lines();

        // TODO: will block on each iteration during the `Read`
        Ok(stream::iter(lines))
    })
    .await
}

pub async fn read_dir<T>(path: T) -> io::Result<impl Stream<Item = io::Result<fs::DirEntry>>>
where
    T: AsRef<Path> + Send,
{
    spawn(move || {
        let entries = fs::read_dir(path)?;

        // TODO: will block on each iteration during the `Read`
        Ok(stream::iter(entries))
    })
    .await
}
