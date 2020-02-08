use std::fmt;

use heim_common::prelude::*;

use crate::{sys, Arch};

/// Host system information.
///
/// ## Provided information
///
/// For example, for Linux host command `uname -a` returns the following line:
/// ```bash
/// $ uname -a
/// Linux tardis 5.0.5-arch1-1-ARCH #1 SMP PREEMPT Wed Mar 27 17:53:10 UTC 2019 x86_64 GNU/Linux
/// ```
///
/// Information in this struct for the same host will look like this:
/// ```text
/// Platform {
///    system: "Linux",
///    release: "5.0.5-arch1-1-ARCH",
///    version: "#1 SMP PREEMPT Wed Mar 27 17:53:10 UTC 2019",
///    hostname: "tardis",
///    architecture: X86_64,
/// }
/// ```
///
/// Windows example:
/// ```text
/// Platform {
///     system: "Windows",
///     release: "10",
///     version: "17763",
///     hostname: "WINDEV1905EVAL",
///     architecture: X86_64,
/// }
/// ```
pub struct Platform(sys::Platform);

wrap!(Platform, sys::Platform);

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

    /// Returns system hostname.
    pub fn hostname(&self) -> &str {
        self.as_ref().hostname()
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
            .field("hostname", &self.hostname())
            .field("architecture", &self.architecture())
            .finish()
    }
}

/// Returns `Future` which resolves into [Platform] struct.
///
/// [Platform]: ./struct.Platform.html
pub fn platform() -> impl Future<Output = Result<Platform>> {
    sys::platform().map_ok(Into::into)
}
