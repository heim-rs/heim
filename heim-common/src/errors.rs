use std::borrow::Cow;
use std::convert;
use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::num;
use std::path::PathBuf;
use std::result;

#[cfg(feature = "backtrace")]
use backtrace::Backtrace;

/// A specialized Result type for data fetching functions.
///
/// This type is used almost for all `heim` routines
/// for any operation which may produce an error.
pub type Result<T> = result::Result<T, Error>;

/// Error details.
#[doc(hidden)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Context {
    /// Invalid data format, unable to parse file.
    File {
        /// File path
        path: PathBuf,
    },
    /// Invalid data format, unable to find required key.
    MissingKey {
        /// Name of the missing key.
        name: Cow<'static, str>,
        /// Data source details.
        source: Cow<'static, str>,
    },
    /// `sysctl(3)` call failed.
    SysCtl {
        /// sysctl `name` parameter value.
        name: Vec<libc::c_int>,
    },
    /// `sysctlbyname(3)` call failed.
    SysCtlByName {
        /// sysctlbyname `name parameter value
        name: ffi::CString,
    },
    /// `sysconf(3)` call failed.
    SysConf {
        /// `sysconf` `name` parameter value.
        name: libc::c_int,
    },
    /// FFI function call failed.
    Ffi {
        /// Name of the called function.
        func: Cow<'static, str>,
    },
    /// Human-readable details for error.
    Message {
        /// Details text.
        text: Cow<'static, str>,
    },
}

/// Error type for data fetching operations.
///
/// Errors are originated from the underlying OS, data parsing
/// or FFI call errors, and it should be assumed that this error
/// is unrecoverable and data can't be fetched at all.
///
/// Note: users **should not** rely on any internal API of this struct,
/// as it is a subject of change in any moment.
#[derive(Debug)]
pub struct Error {
    source: io::Error,
    #[cfg(feature = "backtrace")]
    backtrace: Option<Backtrace>,
    context: Option<Context>,
}

impl Error {
    /// Returns an error backtrace if any.
    #[cfg_attr(docsrs, doc(cfg(all(feature = "unstable", feature = "backtrace"))))]
    #[cfg(feature = "backtrace")]
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    /// Create new `Error` instance from `io::Error` and context details.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn new(source: io::Error, context: Context) -> Self {
        Self {
            source,
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
            context: Some(context),
        }
    }

    /// Returns error representing last OS error that occurred.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn last_os_error() -> Self {
        Self {
            source: io::Error::last_os_error(),
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
            context: None,
        }
    }

    /// Returns internal OS error kind.
    #[doc(hidden)]
    pub fn kind(&self) -> io::ErrorKind {
        self.source.kind()
    }

    /// Creates a new instance of an `Error` from a particular OS error code.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn from_raw_os_error(code: i32) -> Self {
        Self {
            source: io::Error::from_raw_os_error(code),
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
            context: None,
        }
    }

    /// Returns error representing missing key in some data.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn missing_key<K, S>(name: K, source: S) -> Self
    where
        K: Into<Cow<'static, str>>,
        S: Into<Cow<'static, str>>,
    {
        Self {
            source: io::Error::from(io::ErrorKind::InvalidData),
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
            context: Some(Context::MissingKey {
                name: name.into(),
                source: source.into(),
            }),
        }
    }

    /// Return error context if any.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    /// Return mutable reference to the error context.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn context_mut(&mut self) -> &mut Option<Context> {
        &mut self.context
    }

    /// Returns the OS error if this error represents any.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn raw_os_error(&self) -> Option<i32> {
        self.source.raw_os_error()
    }

    /// Returns reference to the inner `io::Error`.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn as_inner(&self) -> &io::Error {
        &self.source
    }

    //
    // Context modifiers
    //

    /// Replace error context with `Context::SysCtl` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_sysctl<T>(mut self, name: T) -> Self
    where
        T: Into<Vec<libc::c_int>>,
    {
        self.context = Some(Context::SysCtl { name: name.into() });
        self
    }

    /// Replace error context with `Context::SysCtlByName` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_sysctlbyname<T>(mut self, name: T) -> Self
    where
        T: Into<ffi::CString>,
    {
        self.context = Some(Context::SysCtlByName { name: name.into() });
        self
    }

    /// Replace error context with `Context::SysConf` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_sysconf<T>(mut self, name: T) -> Self
    where
        T: Into<libc::c_int>,
    {
        self.context = Some(Context::SysConf { name: name.into() });
        self
    }

    /// Replace error context with `Context::Ffi` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_ffi<T>(mut self, name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.context = Some(Context::Ffi { func: name.into() });
        self
    }

    /// Replace error context with `Context::File` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_file<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context = Some(Context::File { path: path.into() });

        self
    }

    /// Replace error context with `Context::Message` instance.
    ///
    /// This method is considered to be an internal API
    /// and should not be used by external parties.
    #[doc(hidden)]
    pub fn with_message<T>(mut self, text: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.context = Some(Context::Message { text: text.into() });

        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.context {
            Some(Context::SysCtl { ref name }) => {
                f.write_fmt(format_args!("sysctl \"{:?}\" failed", name))
            }
            Some(Context::SysCtlByName { ref name }) => {
                f.write_fmt(format_args!("sysctlbyname \"{:?}\" failed", name))
            }
            Some(Context::SysConf { name }) => {
                f.write_fmt(format_args!("sysconf \"{}\" failed", name))
            }
            Some(Context::MissingKey { name, source }) => {
                // TODO: That's ugly
                if source.len() > 0 {
                    f.write_fmt(format_args!(
                        "Unable to find required key \"{}\" in \"{}\"",
                        name, source
                    ))
                } else {
                    f.write_fmt(format_args!("Unable to find required key \"{}\"", name))
                }
            }
            Some(Context::File { path }) => f.write_fmt(format_args!(
                "Unable to parse \"{}\", unsupported format",
                path.display()
            )),
            Some(Context::Message { text }) => f.write_str(text.as_ref()),
            Some(Context::Ffi { func }) => {
                f.write_fmt(format_args!("FFI function \"{}\" call failed", func))
            }
            None => return fmt::Display::fmt(&self.source, f),
        }?;

        f.write_str(": ")?;
        fmt::Display::fmt(&self.source, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            source: e,
            #[cfg(feature = "backtrace")]
            backtrace: Some(Backtrace::new()),
            context: None,
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        let inner = io::Error::new(io::ErrorKind::InvalidData, e);
        Self::from(inner)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        let inner = io::Error::new(io::ErrorKind::InvalidData, e);
        Self::from(inner)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        let inner = io::Error::new(io::ErrorKind::InvalidData, e);
        Self::from(inner)
    }
}

// See https://github.com/heim-rs/heim/issues/182
impl From<convert::Infallible> for Error {
    fn from(e: convert::Infallible) -> Self {
        match e {}
    }
}

#[cfg(unix)]
impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        let inner = match e {
            nix::Error::Sys(errno) => io::Error::from_raw_os_error(errno as i32),
            nix::Error::InvalidPath => io::Error::new(io::ErrorKind::InvalidInput, e),
            nix::Error::InvalidUtf8 => io::Error::new(io::ErrorKind::InvalidData, e),
            nix::Error::UnsupportedOperation => io::Error::new(io::ErrorKind::Other, e),
        };

        Error::from(inner)
    }
}
