use std::io;
use std::path::Path;

use async_std::fs;
use async_std::io::BufReader;
use async_std::prelude::*;
use futures_util::stream::Stream;

// Re-exports
pub use async_std::fs::{read_dir, read_link, read_to_string, DirEntry, File};

/// Returns future which checks if `path` exists
pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    match fs::metadata(path).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

/// Returns stream with the file lines
pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

/// Returns the file first line
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer).await?;

    Ok(buffer)
}
