use std::convert::TryFrom;
use std::io;

/// POSIX signals.
///
/// Signals list is based on the [POSIX.1-2017] specification.
///
/// [POSIX.1-2017]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/signal.h.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Signal {
    /// Process abort signal.
    Abrt,
    /// Alarm clock.
    Alrm,
    /// Access to an undefined portion of a memory object.
    Bus,
    /// Child process terminated, stopped, or continued.
    Chld,
    /// Continue executing, if stopped.
    Cont,
    /// Erroneous arithmetic operation.
    Fpe,
    /// Hangup.
    Hup,
    /// Illegal instruction.
    Ill,
    /// Terminal interrupt signal.
    Int,
    /// Kill (cannot be caught or ignored).
    Kill,
    /// Write on a pipe with no one to read it.
    Pipe,
    /// Terminal quit signal.
    Quit,
    /// Invalid memory reference.
    Segv,
    /// Stop executing (cannot be caught or ignored).
    Stop,
    /// Termination signal.
    Term,
    /// Terminal stop signal.
    Tstp,
    /// Background process attempting read.
    Ttin,
    /// Background process attempting write.
    Ttou,
    /// User-defined signal 1.
    Usr1,
    /// User-defined signal 2.
    Usr2,
    /// Pollable event.
    #[cfg(not(target_os = "macos"))]
    Poll,
    /// Profiling timer expired.
    Prof,
    /// Bad system call.
    Sys,
    /// Trace/breakpoint trap.
    Trap,
    /// High bandwidth data is available at a socket.
    Urg,
    /// Virtual timer expired.
    VtAlrm,
    /// CPU time limit exceeded.
    XCpu,
    /// File size limit exceeded.
    XFsz,
}

impl TryFrom<libc::c_int> for Signal {
    type Error = io::Error;

    fn try_from(value: libc::c_int) -> io::Result<Self> {
        match value {
            libc::SIGABRT => Ok(Signal::Abrt),
            libc::SIGALRM => Ok(Signal::Alrm),
            libc::SIGBUS => Ok(Signal::Bus),
            libc::SIGCHLD => Ok(Signal::Chld),
            libc::SIGCONT => Ok(Signal::Cont),
            libc::SIGFPE => Ok(Signal::Fpe),
            libc::SIGHUP => Ok(Signal::Hup),
            libc::SIGILL => Ok(Signal::Ill),
            libc::SIGINT => Ok(Signal::Int),
            libc::SIGKILL => Ok(Signal::Kill),
            libc::SIGPIPE => Ok(Signal::Pipe),
            libc::SIGQUIT => Ok(Signal::Quit),
            libc::SIGSEGV => Ok(Signal::Segv),
            libc::SIGSTOP => Ok(Signal::Stop),
            libc::SIGTERM => Ok(Signal::Term),
            libc::SIGTSTP => Ok(Signal::Tstp),
            libc::SIGTTIN => Ok(Signal::Ttin),
            libc::SIGTTOU => Ok(Signal::Ttou),
            libc::SIGUSR1 => Ok(Signal::Usr1),
            libc::SIGUSR2 => Ok(Signal::Usr2),
            #[cfg(not(target_os = "macos"))]
            libc::SIGPOLL => Ok(Signal::Poll),
            libc::SIGPROF => Ok(Signal::Prof),
            libc::SIGSYS => Ok(Signal::Sys),
            libc::SIGTRAP => Ok(Signal::Trap),
            libc::SIGURG => Ok(Signal::Urg),
            libc::SIGVTALRM => Ok(Signal::VtAlrm),
            libc::SIGXCPU => Ok(Signal::XCpu),
            libc::SIGXFSZ => Ok(Signal::XFsz),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

impl From<Signal> for libc::c_int {
    fn from(signal: Signal) -> Self {
        match signal {
            Signal::Abrt => libc::SIGABRT,
            Signal::Alrm => libc::SIGALRM,
            Signal::Bus => libc::SIGBUS,
            Signal::Chld => libc::SIGCHLD,
            Signal::Cont => libc::SIGCONT,
            Signal::Fpe => libc::SIGFPE,
            Signal::Hup => libc::SIGHUP,
            Signal::Ill => libc::SIGILL,
            Signal::Int => libc::SIGINT,
            Signal::Kill => libc::SIGKILL,
            Signal::Pipe => libc::SIGPIPE,
            Signal::Quit => libc::SIGQUIT,
            Signal::Segv => libc::SIGSEGV,
            Signal::Stop => libc::SIGSTOP,
            Signal::Term => libc::SIGTERM,
            Signal::Tstp => libc::SIGTSTP,
            Signal::Ttin => libc::SIGTTIN,
            Signal::Ttou => libc::SIGTTOU,
            Signal::Usr1 => libc::SIGUSR1,
            Signal::Usr2 => libc::SIGUSR2,
            #[cfg(not(target_os = "macos"))]
            Signal::Poll => libc::SIGPOLL,
            Signal::Prof => libc::SIGPROF,
            Signal::Sys => libc::SIGSYS,
            Signal::Trap => libc::SIGTRAP,
            Signal::Urg => libc::SIGURG,
            Signal::VtAlrm => libc::SIGVTALRM,
            Signal::XCpu => libc::SIGXCPU,
            Signal::XFsz => libc::SIGXFSZ,
        }
    }
}
