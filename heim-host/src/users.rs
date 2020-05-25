use std::fmt;

use crate::sys;
use heim_common::prelude::*;

/// User currently connected to system.
///
/// See [os] module for OS-specific extensions.
///
/// [os]: ./os/index.html
pub struct User(sys::User);

wrap!(User, sys::User);

impl User {
    /// Returns the name of user.
    pub fn username(&self) -> &str {
        self.as_ref().username()
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username())
            .finish()
    }
}

/// Returns a stream over [User] sessions currently connected to the system.
///
/// ## Compatibility
///
/// For `musl` target environment this stream always will be empty,
/// see [#141](https://github.com/heim-rs/heim/issues/141).
///
/// [User]: ./struct.User.html
pub async fn users() -> Result<impl Stream<Item = Result<User>>> {
    let inner = sys::users().await?;

    Ok(inner.map_ok(Into::into))
}
