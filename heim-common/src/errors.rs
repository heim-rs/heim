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

#[derive(Debug)]
pub enum LoadFailureReason {
    MissingEntity(Cow<'static, str>),
    Incompatible(&'static str),
    Other(Box<dyn error::Error + Send + 'static>),

    #[doc(hidden)]
    __Nonexhaustive,
}

/// Error which might happen while calling `heim` API.
#[derive(Debug)]
pub enum Error {
    /// Any error which may happen during the data fetch.
    ///
    /// Usually means that `heim` is not compatible
    /// with a system it's running on.
    ///
    /// Caller should consider it as an opaque enum member
    /// and use the data in it only for debugging reasons.
    /// Contents of this enum members are not stable and may
    /// change without any warning.
    LoadFailure(LoadFailureReason),
}

impl Error {
    #[doc(hidden)]
    pub fn last_os_error() -> Error {
        let e = Box::new(io::Error::last_os_error());
        Error::LoadFailure(LoadFailureReason::Other(e))
    }

    #[doc(hidden)]
    pub fn missing_entity<T: Into<Cow<'static, str>>>(name: T) -> Error {
        Error::LoadFailure(LoadFailureReason::MissingEntity(name.into()))
    }

    #[doc(hidden)]
    pub fn incompatible(desc: &'static str) -> Error {
        Error::LoadFailure(LoadFailureReason::Incompatible(desc))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LoadFailure(LoadFailureReason::MissingEntity(name)) => {
                f.write_fmt(format_args!("Expected entity `{}` is missing", name))
            }
            Error::LoadFailure(LoadFailureReason::Incompatible(reason)) => f.write_str(reason),
            Error::LoadFailure(LoadFailureReason::Other(e)) => fmt::Display::fmt(e, f),
            _ => f.write_str("Unknown error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::LoadFailure(LoadFailureReason::Other(e)) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<ffi::IntoStringError> for Error {
    fn from(e: ffi::IntoStringError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<string::ParseError> for Error {
    fn from(e: string::ParseError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl From<net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}

impl<T> From<Box<T>> for Error
where
    T: error::Error + Send + 'static,
{
    fn from(e: Box<T>) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(e))
    }
}

#[cfg(unix)]
impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error::LoadFailure(LoadFailureReason::Other(Box::new(e)))
    }
}
