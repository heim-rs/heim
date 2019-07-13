//! Internal extensions for `Iterator`s.

use std::convert::TryFrom;
use std::io;
use std::str::FromStr;

use crate::{Error, Result};

/// Extension trait for all `T: Iterator`.
///
/// Used across the `heim` sub-crates only.
pub trait TryIterator: Iterator {
    /// Attempt to fetch next element from iterator,
    /// but instead of returning `Option<T>` returns `Result<T>`.
    fn try_next(&mut self) -> Result<<Self as Iterator>::Item>;

    /// Attempt to fetch next element from iterator
    /// and try to convert it into `R` type.
    ///
    /// Type `R` should implement `TryFrom<Iterator::Item>`.
    fn try_from_next<R, E>(&mut self) -> Result<R>
    where
        R: TryFrom<<Self as Iterator>::Item, Error = E>,
        Error: From<E>;
}

/// Extension trait for all `T: Iterator`.
///
/// Used across the `heim` sub-crates only.
pub trait ParseIterator<I>: TryIterator<Item = I>
where
    I: AsRef<str>,
{
    /// Attempt to to parse next yielded element from the iterator.
    ///
    /// Type `R` should implement `std::str::FromStr` trait in order
    /// to be able parsed from the iterator element.
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
        self.next()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))
            .map_err(Into::into)
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

impl<T, I> ParseIterator<I> for T
where
    T: TryIterator<Item = I>,
    I: AsRef<str>,
{
    fn try_parse_next<R, E>(&mut self) -> Result<R>
    where
        R: FromStr<Err = E>,
        Error: From<E>,
    {
        let value = self.try_next()?;

        FromStr::from_str(value.as_ref()).map_err(Into::into)
    }
}
