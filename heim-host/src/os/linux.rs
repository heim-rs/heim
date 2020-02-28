//! Linux-specific extensions.

use std::net::IpAddr;

use crate::Pid;

cfg_if::cfg_if! {
    // aarch64-unknown-linux-gnu has different type
    if #[cfg(all(target_arch = "aarch64", not(target_family = "musl")))] {
        /// User session ID.
        pub type SessionId = i64;
    } else {
        /// User session ID.
        pub type SessionId = i32;
    }
}
//
///// User session ID.
//#[cfg(not(target_arch = "aarch64"))]
//pub type SessionId = i32;
//
///// User session ID.
//#[cfg(target_arch = "aarch64")]
//pub type SessionId = i64;

/// Linux-specific extensions for [User].
///
/// In Linux user information is provided by `utmpx` (see `man utmpx(5)`),
/// trait methods are representing fields of this struct.
///
/// [User]: ../../struct.User.html
pub trait UserExt {
    /// Returns the `Pid` of login process.
    fn pid(&self) -> Pid;

    /// Returns the tty or pseudo-tty name associated with user.
    fn terminal(&self) -> &str;

    /// Returns the terminal identifier.
    fn id(&self) -> &str;

    /// Returns the hostname for remote login.
    fn hostname(&self) -> &str;

    /// Returns the IP address of remote host.
    fn address(&self) -> Option<IpAddr>;

    /// Returns the Session ID.
    ///
    /// ## Compatibility
    ///
    /// Note that session id type is not portable
    /// and varies depending on target architecture.
    fn session_id(&self) -> SessionId;
}

#[cfg(target_os = "linux")]
impl UserExt for crate::User {
    fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    fn terminal(&self) -> &str {
        self.as_ref().terminal()
    }

    fn id(&self) -> &str {
        self.as_ref().id()
    }

    fn hostname(&self) -> &str {
        self.as_ref().hostname()
    }

    fn address(&self) -> Option<IpAddr> {
        self.as_ref().address()
    }

    fn session_id(&self) -> SessionId {
        self.as_ref().session_id()
    }
}
