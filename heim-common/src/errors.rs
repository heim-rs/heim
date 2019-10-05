#![allow(deprecated)] // TODO: Temporary, while switching from `Error` to `Error2`

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
#[deprecated]
pub type Result<T> = result::Result<T, Error>;

/// NG: Type alias for types returned by `heim` functions.
pub type Result2<T> = result::Result<T, Error2>;

#[derive(Debug)]
#[allow(missing_docs)]
pub enum ErrorContext {
    Message(Cow<'static, str>),
    NamedSyscall { name: Cow<'static, str> },
    Syscall { num: libc::c_int },
    SysCtl { name: Vec<libc::c_int> },
    Ffi { func: Cow<'static, str> },
}

/// NG: Any error which may happen during the data fetching.
pub struct Error2 {
    source: io::Error,
    context: Option<ErrorContext>,
}

impl Error2 {
    #[doc(hidden)]
    pub fn new(source: io::Error, ctx: ErrorContext) -> Self {
        Error2 {
            source,
            context: Some(ctx),
        }
    }

    #[doc(hidden)]
    pub fn last_os_error() -> Error2 {
        Error2::from(io::Error::last_os_error())
    }

    #[doc(hidden)]
    pub fn raw_os_error(&self) -> Option<i32> {
        self.source.raw_os_error()
    }

    #[doc(hidden)]
    pub fn kind(&self) -> io::ErrorKind {
        self.source.kind()
    }

    #[doc(hidden)]
    pub fn context(mut self, ctx: ErrorContext) -> Self {
        self.context = Some(ctx);

        self
    }

    #[doc(hidden)]
    pub fn with_message(self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.context(ErrorContext::Message(msg.into()))
    }

    #[doc(hidden)]
    pub fn with_syscall(self, num: impl Into<libc::c_int>) -> Self {
        self.context(ErrorContext::Syscall { num: num.into() })
    }

    #[doc(hidden)]
    pub fn with_named_syscall(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.context(ErrorContext::NamedSyscall { name: name.into() })
    }

    #[doc(hidden)]
    pub fn with_ffi(self, func: impl Into<Cow<'static, str>>) -> Self {
        self.context(ErrorContext::Ffi { func: func.into() })
    }

    #[doc(hidden)]
    pub fn with_sysctl(self, name: &[libc::c_int]) -> Self {
        self.context(ErrorContext::SysCtl {
            name: Vec::from(name),
        })
    }
}

impl fmt::Debug for Error2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt = f.debug_struct("Error");

        match &self.context {
            None => {}
            Some(ErrorContext::Message(..)) => {}
            Some(ErrorContext::Syscall { num }) => {
                let _ = fmt.field("syscall", &num);
            }
            Some(ErrorContext::NamedSyscall { name }) => {
                let _ = fmt.field("syscall", &name);
            }
            Some(ErrorContext::Ffi { func }) => {
                let _ = fmt.field("ffi_function", &func);
            }
            Some(ErrorContext::SysCtl { name }) => {
                let _ = fmt.field("sysctl", &name);
            }
        };

        fmt.field("source", &self.source).finish()
    }
}

impl fmt::Display for Error2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.context {
            None => fmt::Display::fmt(&self.source, f),
            Some(ErrorContext::Message(msg)) => f.write_str(&msg),
            Some(ErrorContext::Syscall { num }) => {
                f.write_fmt(format_args!("Failed to invoke the `{}` syscall", num,))
            }
            Some(ErrorContext::NamedSyscall { name }) => {
                f.write_fmt(format_args!("Failed to invoke the `{}` syscall", name,))
            }
            Some(ErrorContext::SysCtl { name }) => {
                f.write_fmt(format_args!("Failed to invoke the `{:?}` sysctl", name))
            }
            Some(ErrorContext::Ffi { func }) => {
                f.write_fmt(format_args!("Failed to call a FFI function `{}`", func))
            }
        }
    }
}

impl error::Error for Error2 {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<io::Error> for Error2 {
    fn from(e: io::Error) -> Error2 {
        Error2 {
            source: e,
            context: None,
        }
    }
}

impl From<num::ParseFloatError> for Error2 {
    fn from(e: num::ParseFloatError) -> Error2 {
        io::Error::new(io::ErrorKind::InvalidData, e).into()
    }
}

impl From<Error2> for io::Error {
    fn from(e: Error2) -> io::Error {
        e.source
    }
}

impl From<Error> for Error2 {
    fn from(e: Error) -> Error2 {
        match e {
            Error::MissingEntity(name) => io::Error::new(io::ErrorKind::InvalidInput, name).into(),
            Error::Incompatible(message) => io::Error::new(io::ErrorKind::Other, message).into(),
            Error::Io(e) => e.into(),
            Error::Other(_e) => io::Error::from(io::ErrorKind::Other).into(),
            _ => unreachable!(),
        }
    }
}

impl From<num::ParseIntError> for Error2 {
    fn from(e: num::ParseIntError) -> Error2 {
        io::Error::new(io::ErrorKind::InvalidData, e).into()
    }
}

/// Any error which may happen during the data fetch.
///
/// Usually means that `heim` is not compatible
/// with a system it's running on.
///
/// Users should consider this enum as opaque type (kinda like with `Box<dyn Error>`)
/// and use the data in it only for debugging reasons.
/// Contents of this enum are not stable and may
/// change without any warning.
#[derive(Debug)]
#[deprecated]
pub enum Error {
    #[doc(hidden)]
    MissingEntity(Cow<'static, str>),
    #[doc(hidden)]
    Incompatible(Cow<'static, str>),
    #[doc(hidden)]
    Io(io::Error),
    #[doc(hidden)]
    Other(Box<dyn error::Error + Send + 'static>),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Error {
    #[doc(hidden)]
    pub fn last_os_error() -> Error {
        Error::from(io::Error::last_os_error())
    }

    #[doc(hidden)]
    pub fn raw_os_error(&self) -> Option<i32> {
        match self {
            Error::Io(e) => e.raw_os_error(),
            _ => None,
        }
    }

    #[doc(hidden)]
    pub fn missing_entity<T: Into<Cow<'static, str>>>(name: T) -> Error {
        Error::MissingEntity(name.into())
    }

    #[doc(hidden)]
    pub fn incompatible<T: Into<Cow<'static, str>>>(desc: T) -> Error {
        Error::Incompatible(desc.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingEntity(name) => {
                f.write_fmt(format_args!("Expected entity `{}` is missing", name))
            }
            Error::Incompatible(reason) => f.write_str(reason),
            Error::Io(e) => fmt::Display::fmt(e, f),
            Error::Other(e) => fmt::Display::fmt(e, f),
            _ => f.write_str("Unknown error"),
        }
    }
}

impl From<Error2> for Error {
    fn from(e: Error2) -> Error {
        Error::Io(e.into())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(&*e),
            Error::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
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
