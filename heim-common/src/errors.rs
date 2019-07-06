use std::borrow::Cow;
use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::net;
use std::num;
use std::result;
use std::string;

/// Type alias for types returned by `heim` functions.
pub type Result<T> = result::Result<T, Error>;

/// Any error which may happen during the data fetch.
///
/// Usually means that `heim` is not compatible
/// with a system it's running on.
///
/// Users should consider this enum as opaque type (like with `Box<dyn Error>`)
/// and use the data in it only for debugging reasons.
/// Contents of this enum are not stable and may
/// change without any warning.
#[derive(Debug)]
pub enum Error {
    MissingEntity(Cow<'static, str>),
    Incompatible(&'static str),
    Other(Box<dyn error::Error + Send + 'static>),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Error {
    #[doc(hidden)]
    pub fn last_os_error() -> Error {
        let e = Box::new(io::Error::last_os_error());
        Error::Other(e)
    }

    #[doc(hidden)]
    pub fn missing_entity<T: Into<Cow<'static, str>>>(name: T) -> Error {
        Error::MissingEntity(name.into())
    }

    #[doc(hidden)]
    pub fn incompatible(desc: &'static str) -> Error {
        Error::Incompatible(desc)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingEntity(name) => f.write_fmt(format_args!("Expected entity `{}` is missing", name)),
            Error::Incompatible(reason) => f.write_str(reason),
            Error::Other(e) => fmt::Display::fmt(e, f),
            _ => f.write_str("Unknown error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<ffi::IntoStringError> for Error {
    fn from(e: ffi::IntoStringError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<string::ParseError> for Error {
    fn from(e: string::ParseError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl<T> From<Box<T>> for Error
where
    T: error::Error + Send + 'static,
{
    fn from(e: Box<T>) -> Self {
        Error::Other(e)
    }
}

#[cfg(unix)]
impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error::Other(Box::new(e))
    }
}
