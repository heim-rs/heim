//! Unix-specific extensions
use crate::Pid;

/// Unix-specific extensions for [User].
///
/// This trait is not implemented for [User] for `OpenBSD` target.
///
/// [User]: ../../struct.User.html
#[heim_derive::os_ext_for(crate::User, cfg(all(unix, not(target_os = "openbsd"))))]
pub trait UserExt {
    /// Returns the user `Pid`.
    fn pid(&self) -> Pid;

    /// Returns the tty or pseudo-tty name associated with user.
    fn terminal(&self) -> Option<&str>;
}
