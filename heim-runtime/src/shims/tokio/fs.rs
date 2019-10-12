use std::io;
use std::path::Path;

use futures_util::stream::Stream;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

// Re-exports
pub use tokio::fs::{read_dir, read_link, File};
pub use tokio_fs::DirEntry;

pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    match fs::metadata(path).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path).await?;
    let mut buffer = String::with_capacity(128);
    // TODO: Same to `std`, read the capacity from the file metadata
    let _ = file.read_to_string(&mut buffer).await?;

    Ok(buffer)
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

/// Returns future which tries to read the first line from file.
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).await?;

    Ok(buffer)
}
