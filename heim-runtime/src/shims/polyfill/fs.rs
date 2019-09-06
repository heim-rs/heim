use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[cfg(target_os = "windows")]
use std::os::windows::io::{AsRawHandle, RawHandle};

use heim_common::prelude::{
    futures::{future, stream},
    Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt,
};

use super::pool::THREAD_POOL;

#[derive(Debug)]
pub struct File(fs::File);

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl File {
    pub fn open<T>(path: T) -> impl Future<Output = io::Result<File>>
    where
        T: AsRef<Path> + Send + Unpin + 'static,
    {
        THREAD_POOL.spawn(|| fs::File::open(path).map(File))
    }

    #[cfg(target_os = "windows")]
    pub fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}

pub fn path_exists<T>(path: T) -> impl Future<Output = bool>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    THREAD_POOL.spawn(move || path.as_ref().exists())
}

pub fn read_to_string<T>(path: T) -> impl Future<Output = io::Result<String>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    THREAD_POOL.spawn(move || fs::read_to_string(path))
}

pub fn read_lines<T>(path: T) -> impl Stream<Item = io::Result<String>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    File::open(path)
        .map_ok(|file| {
            let reader = io::BufReader::new(file);
            stream::iter(reader.lines())
        })
        .try_flatten_stream()
        .into_stream()
}

pub fn read_first_line<T>(path: T) -> impl Future<Output = io::Result<String>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    // TODO: Looks dumb
    read_lines(path)
        .into_stream()
        .into_future()
        .map(|(try_line, _)| match try_line {
            Some(Ok(line)) => Ok(line),
            Some(Err(e)) => Err(e),
            None => Err(io::Error::from(io::ErrorKind::InvalidData)),
        })
}

pub fn read_link<T>(path: T) -> impl Future<Output = io::Result<PathBuf>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    THREAD_POOL.spawn(move || fs::read_link(path))
}

pub fn read_dir<T>(path: T) -> impl Stream<Item = io::Result<fs::DirEntry>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    THREAD_POOL
        .spawn(move || fs::read_dir(path))
        .map_ok(stream::iter)
        .try_flatten_stream()
}

pub fn read_into<T, R, E>(path: T) -> impl Future<Output = Result<R, E>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    read_to_string(path)
        .map_err(E::from)
        .and_then(|content| future::ready(R::from_str(&content).map_err(Into::into)))
}

pub fn read_lines_into<T, R, E>(path: T) -> impl Stream<Item = Result<R, E>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
    R: FromStr<Err = E>,
    E: From<io::Error>,
{
    read_lines(path).map_err(E::from).then(|result| {
        let res = result.and_then(|line| R::from_str(&line));

        future::ready(res)
    })
}
