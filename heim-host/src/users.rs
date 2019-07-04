use std::fmt;

use crate::sys;
use heim_common::prelude::*;

/// User currently connected to system.
///
/// See [os] module for OS-specific extensions.
///
/// [os]: ./os/
#[derive(heim_derive::ImplWrap)]
pub struct User(sys::User);

impl User {
    /// Returns the name of user.
    pub fn username(&self) -> &str {
        self.as_ref().username()
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User").field("username", &self.username()).finish()
    }
}

/// Returns stream which yields [User]s.
pub fn users() -> impl Stream<Item = Result<User>> {
    sys::users().map_ok(Into::into)
}
