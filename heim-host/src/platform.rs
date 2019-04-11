use std::fmt;

use heim_common::prelude::*;

use crate::{sys, Arch};

/// Host system information.
#[derive(heim_derive::ImplWrap)]
pub struct Platform(sys::Platform);

impl Platform {
    /// Returns system name.
    pub fn system(&self) -> &str {
        self.as_ref().system()
    }

    /// Returns system release.
    pub fn release(&self) -> &str {
        self.as_ref().release()
    }

    /// Returns system version.
    pub fn version(&self) -> &str {
        self.as_ref().version()
    }

    /// Returns system architecture.
    pub fn architecture(&self) -> Arch {
        self.as_ref().architecture()
    }
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Platform")
            .field("system", &self.system())
            .field("release", &self.release())
            .field("version", &self.version())
            .field("architecture", &self.architecture())
            .finish()
    }
}

/// Returns `Future` which resolves into [Platform] struct.
pub fn platform() -> impl Future<Item = Platform, Error = Error> {
    sys::platform().map(Into::into)
}
