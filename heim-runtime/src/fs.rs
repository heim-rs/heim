use std::io;
use std::path::Path;
use std::str::FromStr;

use futures::{io::BufReader, AsyncBufReadExt, Stream, StreamExt, TryStreamExt};
use smol::unblock;

// Public re-exports
pub use smol::fs::{read, read_dir, read_link, read_to_string, File};

pub async fn path_exists<T>(path: T) -> bool
where
    T: AsRef<Path> + Send + 'static,
{
    let path = path.as_ref().to_owned();
    unblock(move || path.exists()).await
}

pub async fn read_into<T, R, E>(path: T) -> Result<R, E>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr + Send + 'static,
    E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
    let contents = read_to_string(path).await?;

    R::from_str(&contents).map_err(Into::into)
}

pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
where
    T: AsRef<Path> + Send + 'static,
{
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub async fn read_lines_into<T, R, E>(path: T) -> io::Result<impl Stream<Item = Result<R, E>>>
where
    T: AsRef<Path> + Send + 'static,
    R: FromStr + Send + 'static,
    E: From<io::Error> + From<<R as FromStr>::Err> + Send + 'static,
{
    let stream = read_lines(path).await?;
    let stream = stream
        .map_err(E::from)
        .and_then(|line| async move { R::from_str(&line).map_err(E::from) });

    Ok(stream)
}

pub async fn read_first_line<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path> + Send + 'static,
{
    match read_lines(path).await?.next().await {
        Some(Ok(line)) => Ok(line),
        Some(Err(e)) => Err(e),
        None => Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}
