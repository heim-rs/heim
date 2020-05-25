use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use futures::io::{AsyncBufReadExt, BufReader};
use futures::Stream;

use crate::spawn_blocking;

pub async fn read_to_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || fs::read_to_string(path)).await
}

pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr + Send + 'static,
    E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
    spawn_blocking(|| {
        let contents = fs::read_to_string(path)?;

        R::from_str(&contents).map_err(Into::into)
    })
    .await
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path> + Send + 'static,
{
    let file = spawn_blocking(move || fs::File::open(path)).await?;

    let stream = smol::reader(file);
    let reader = BufReader::new(stream);
    Ok(reader.lines())
}

pub async fn read_lines_into<T, R, E>(path: T) -> io::Result<impl Stream<Item = Result<R, E>>>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr + Send + 'static,
    E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
    let lines = spawn_blocking(move || {
        let file = fs::File::open(path)?;

        let reader = io::BufReader::new(file);
        let lines = reader.lines();

        Ok::<_, io::Error>(lines)
    })
    .await?;

    let iter = lines.map(|try_line| match try_line {
        Ok(line) => R::from_str(&line).map_err(E::from),
        Err(e) => Err(E::from(e)),
    });

    Ok(smol::iter(iter))
}

pub async fn read<T>(path: T) -> io::Result<Vec<u8>>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || fs::read(path)).await
}

pub async fn read_link<T>(path: T) -> io::Result<PathBuf>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || fs::read_link(path)).await
}

pub async fn read_dir<T>(path: T) -> io::Result<impl Stream<Item = io::Result<fs::DirEntry>>>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || {
        let entries = fs::read_dir(path)?;
        // TODO: Might move iterator into another thread,
        // would nice to continue execution on the same thread.
        Ok(smol::iter(entries))
    })
    .await
}

pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(move || path.as_ref().exists()).await
}

pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send + 'static,
{
    spawn_blocking(|| {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        match lines.next() {
            Some(Ok(line)) => Ok(line),
            Some(Err(e)) => Err(e),
            None => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    })
    .await
}
