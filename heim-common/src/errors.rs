#![allow(deprecated)] // TODO: Temporary, while switching from `Error` to `Error`

use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;
use std::num;
use std::result;

/// Type alias for types returned by `heim` functions.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
#[allow(missing_docs)]
pub enum ErrorContext {
    Message(Cow<'static, str>),
    NamedSyscall { name: Cow<'static, str> },
    Syscall { num: libc::c_int },
    SysCtl { name: Vec<libc::c_int> },
    Ffi { func: Cow<'static, str> },
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
pub struct Error {
    source: io::Error,
    context: Option<ErrorContext>,
}

impl Error {
    #[doc(hidden)]
    pub fn new(source: io::Error, ctx: ErrorContext) -> Self {
        Error {
            source,
            context: Some(ctx),
        }
    }

    #[doc(hidden)]
    pub fn last_os_error() -> Error {
        Error::from(io::Error::last_os_error())
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

impl fmt::Debug for Error {
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

impl fmt::Display for Error {
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error {
            source: e,
            context: None,
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Error {
        io::Error::new(io::ErrorKind::InvalidData, e).into()
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        e.source
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        io::Error::new(io::ErrorKind::InvalidData, e).into()
    }
}
