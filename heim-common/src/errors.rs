use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::net;
use std::num;
use std::result;
use std::string;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    /// Unable to call system function because program is incompatible with OS used
    Incompatible,
    /// Unable to determine the requested value
    UnknownValue,
    FromNul(ffi::NulError),
    FromFfiString(ffi::IntoStringError),
    Io(io::Error),
    /// Generic parse error. If applicable, try to use more specific ErrorKind
    Parse,
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    ParseString(string::ParseError),
    FromUtf8(string::FromUtf8Error),
    Other(Box<dyn error::Error + Send + 'static>),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
        }
    }

    pub fn last_os_error() -> Error {
        io::Error::last_os_error().into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Incompatible => f.write_str("Incompatible with current OS"),
            ErrorKind::UnknownValue => f.write_str("Unable to determine the value"),
            ErrorKind::FromNul(e) => fmt::Display::fmt(e, f),
            ErrorKind::FromFfiString(e) => fmt::Display::fmt(e, f),
            ErrorKind::Io(e) => fmt::Display::fmt(e, f),
            ErrorKind::Parse => f.write_str("Unable to parse"),
            ErrorKind::ParseInt(e) => fmt::Display::fmt(e, f),
            ErrorKind::ParseFloat(e) => fmt::Display::fmt(e, f),
            ErrorKind::ParseString(e) => fmt::Display::fmt(e, f),
            ErrorKind::FromUtf8(e) => fmt::Display::fmt(e, f),
            ErrorKind::Other(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            kind: ErrorKind::Io(e),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error {
            kind: ErrorKind::ParseInt(e),
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error {
            kind: ErrorKind::ParseFloat(e),
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Error {
            kind: ErrorKind::FromNul(e),
        }
    }
}

impl From<ffi::IntoStringError> for Error {
    fn from(e: ffi::IntoStringError) -> Self {
        Error {
            kind: ErrorKind::FromFfiString(e),
        }
    }
}

impl From<string::ParseError> for Error {
    fn from(e: string::ParseError) -> Self {
        Error {
            kind: ErrorKind::ParseString(e),
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error {
            kind: ErrorKind::FromUtf8(e),
        }
    }
}

impl From<net::AddrParseError> for Error {
    fn from(_e: net::AddrParseError) -> Self {
        Error {
            kind: ErrorKind::Parse,
        }
    }
}

impl<T> From<Box<T>> for Error
where
    T: error::Error + Send + 'static,
{
    fn from(e: Box<T>) -> Self {
        Error {
            kind: ErrorKind::Other(e),
        }
    }
}

#[cfg(unix)]
impl From<nix::Error> for Error {
    fn from(_e: nix::Error) -> Self {
        unimplemented!()
    }
}
