//! macOS-specific extensions.

use heim_common::{Error, Pid, Result, Uid};
use std::convert::TryFrom;

/// macOS-specific extensions for [User].
///
/// In macOS user information is provided by `utmpx` (see `man utmpx(5)`),
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
}

#[cfg(target_os = "macos")]
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
}

#[cfg(target_os = "macos")]
impl TryFrom<Uid> for crate::User {
    type Error = Error;
    fn try_from(uid: Uid) -> Result<Self> {
        let user = crate::sys::User::try_from(uid)?;
        Ok(crate::User::try_from(user)?)
    }
}
