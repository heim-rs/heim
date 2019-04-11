use std::fmt;

use crate::sys;
use heim_common::prelude::*;

/// User currently connected to system.
///
/// See [os] module for OS-specific extensions.
#[derive(heim_derive::ImplWrap)]
pub struct User(sys::User);

impl User {
    /// Returns the name of user.
    pub fn username(&self) -> &str {
        self.as_ref().username()
    }

    /// Returns the tty or pseudo-tty name associated with user.
    pub fn terminal(&self) -> Option<&str> {
        self.as_ref().terminal()
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username())
            .field("terminal", &self.terminal())
            .finish()
    }
}

/// Returns stream which yields [User]
pub fn users() -> impl Stream<Item = User, Error = Error> {
    sys::users().map(Into::into)
}
