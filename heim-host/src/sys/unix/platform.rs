use std::mem;
use std::str::FromStr;

use heim_common::prelude::*;

use crate::Arch;
use super::into_cow;

#[derive(Debug)]
pub struct Platform {
    system: String,
    release: String,
    version: String,
    arch: Arch,
}

impl Platform {
    pub fn system(&self) -> &str {
        &self.system
    }

    pub fn release(&self) -> &str {
        &self.release
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn architecture(&self) -> Arch {
        self.arch
    }
}

// Based on the https://github.com/uutils/platform-info/blob/master/src/unix.rs
pub fn platform() -> impl Future<Output = Result<Platform>> {
    future::lazy(|_| unsafe {
        let mut uts = mem::MaybeUninit::<libc::utsname>::uninit();
        let result = libc::uname(uts.as_mut_ptr());

        if result != 0 {
            Err(Error::last_os_error())
        } else {
            let uts = uts.assume_init();
            let raw_arch = into_cow(&uts.machine);
            let arch = Arch::from_str(&raw_arch).unwrap_or(Arch::Unknown);

            Ok(Platform {
                system: into_cow(&uts.sysname).into_owned(),
                release: into_cow(&uts.release).into_owned(),
                version: into_cow(&uts.version).into_owned(),
                arch,
            })

        }
    })
}
