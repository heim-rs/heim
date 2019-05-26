use std::str::FromStr;
use std::convert::TryFrom;

use crate::{Error, ErrorKind, Result};

pub trait TryIterator: Iterator {
    fn try_next(&mut self) -> Result<<Self as Iterator>::Item>;

    fn try_from_next<R, E>(&mut self) -> Result<R>
    where
        R: TryFrom<<Self as Iterator>::Item, Error = E>,
        Error: From<E>;
}

pub trait ParseIterator<I>: TryIterator<Item = I> where I: AsRef<str> {
    fn try_parse_next<R, E>(&mut self) -> Result<R>
    where
        R: FromStr<Err = E>,
        Error: From<E>;
}

impl<T> TryIterator for T
where
    T: Iterator,
{
    fn try_next(&mut self) -> Result<<Self as Iterator>::Item> {
        self.next().ok_or_else(|| Error::new(ErrorKind::Parse))
    }

    fn try_from_next<R, E>(&mut self) -> Result<R>
    where
        R: TryFrom<<Self as Iterator>::Item, Error = E>,
        Error: From<E>,
    {
        let value = self.try_next()?;

        TryFrom::try_from(value).map_err(Into::into)
    }
}

impl<T, I> ParseIterator<I> for T where T: TryIterator<Item = I>, I: AsRef<str> {
    fn try_parse_next<R, E>(&mut self) -> Result<R> where
        R: FromStr<Err=E>,
        Error: From<E> {

        let value = self.try_next()?;

        FromStr::from_str(value.as_ref()).map_err(Into::into)
    }
}
