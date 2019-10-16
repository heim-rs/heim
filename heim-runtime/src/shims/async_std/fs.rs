use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use async_std::fs;
use async_std::io::BufReader;
use async_std::prelude::*;
use futures_util::stream::Stream;
use futures_util::try_stream::TryStreamExt;

// Re-exports
pub use async_std::fs::File;

/// Newtype direntry
#[derive(Debug)]
pub struct DirEntry(fs::DirEntry);

impl DirEntry {
    /// Returns path
    pub fn path(&self) -> PathBuf {
        self.0.path().into()
    }

    /// Returns file name
    pub fn file_name(&self) -> OsString {
        self.0.file_name().into()
    }
}

/// Read symlink target
pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path>,
{
    let res = fs::read_link(path.as_ref()).await?;
    Ok(res.into())
}

/// Read files in directory
pub async fn read_dir<T>(path: T) -> io::Result<impl Stream<Item = io::Result<DirEntry>>>
where
    T: AsRef<Path>,
{
    let reader = fs::read_dir(path.as_ref()).await?;

    Ok(reader.map_ok(DirEntry))
}

/// Read `path` contents into the UTF-8 string.
pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    fs::read_to_string(path.as_ref()).await
}

/// Returns future which checks if `path` exists
pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    match fs::metadata(path.as_ref()).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

/// Returns stream with the file lines
pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path.as_ref()).await?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

/// Returns the file first line
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path.as_ref()).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer).await?;

    Ok(buffer)
}
