use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use std::os::windows::io::{AsRawHandle, RawHandle};

use futures_util::stream::{self, Stream, StreamExt};

use super::pool::run;

// Re-exports
pub use std::fs::DirEntry;

/// stub
#[derive(Debug)]
pub struct File(fs::File);

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl File {
    /// stub
    pub async fn open<T>(path: T) -> io::Result<File>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref().to_owned();
        run(move || fs::File::open(path).map(File)).await
    }

    /// stub
    #[cfg(target_os = "windows")]
    pub fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}

/// stub
pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    let path = path.as_ref().to_owned();
    run(move || path.exists()).await
}

/// stub
pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let path = path.as_ref().to_owned();
    run(move || fs::read_to_string(path)).await
}

/// stub
pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    let file = File::open(path).await?;

    let reader = io::BufReader::new(file);
    Ok(stream::iter(reader.lines()))
}

/// stub
pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    let mut lines = read_lines(path).await?;
    match lines.next().await {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => Err(e),
        None => Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}

/// stub
pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path>,
{
    let path = path.as_ref().to_owned();
    run(move || fs::read_link(path)).await
}

/// stub
pub async fn read_dir<T>(path: T) -> io::Result<impl Stream<Item = io::Result<fs::DirEntry>>>
where
    T: AsRef<Path>,
{
    let path = path.as_ref().to_owned();
    let reader = run(move || fs::read_dir(path)).await?;

    Ok(stream::iter(reader))
}
