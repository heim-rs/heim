use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use futures::{
    task::{Context, Poll},
    Stream,
};

use async_std::{
    fs::{self, File},
    io::prelude::BufReadExt,
    io::{BufReader, Lines},
};

pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path> + Send,
{
    let path = path.as_ref();

    async_std::path::Path::new(&path).exists().await
}

pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path> + Send,
{
    let path = path.as_ref();
    fs::read_link(path).await.map(|path| path.into())
}

pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send,
{
    let path = path.as_ref();
    fs::read_to_string(path).await
}

pub async fn read_dir<T>(path: T) -> io::Result<ReadDir>
where
    T: AsRef<Path> + Send,
{
    let path = path.as_ref();
    let stream = fs::read_dir(path).await?;

    Ok(ReadDir(stream))
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path).await?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}
pub struct DirEntry(fs::DirEntry);

impl DirEntry {
    pub fn file_name(&self) -> OsString {
        self.0.file_name()
    }

    pub fn path(&self) -> PathBuf {
        self.0.path().into()
    }
}

pub struct ReadDir(fs::ReadDir);

impl futures::Stream for ReadDir {
    type Item = io::Result<DirEntry>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pinned = Pin::new(&mut self.0);
        match futures::ready!(pinned.poll_next(cx)) {
            Some(Ok(entry)) => Poll::Ready(Some(Ok(DirEntry(entry)))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None),
        }
    }
}
