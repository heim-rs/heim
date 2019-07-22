//! macOS-specific extensions.

use crate::Pid;

/// macOS-specific extensions for [User].
///
/// In macOS user information is provided by `utmpx` (see `man utmpx(5)`),
/// trait methods are representing fields of this struct.
///
/// [User]: ../../struct.User.html
#[heim_derive::os_ext_for(crate::User, cfg(target_os = "macos"))]
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
