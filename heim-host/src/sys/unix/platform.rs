use std::ffi::CStr;
use std::mem;
use std::str::FromStr;

use heim_common::prelude::*;

use crate::Arch;

#[derive(Debug)]
pub struct Platform {
    system: String,
    release: String,
    version: String,
    hostname: String,
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

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn architecture(&self) -> Arch {
        self.arch
    }
}

// Based on the https://github.com/uutils/platform-info/blob/master/src/unix.rs
pub async fn platform() -> Result<Platform> {
    let mut uts = mem::MaybeUninit::<libc::utsname>::uninit();
    let result = unsafe { libc::uname(uts.as_mut_ptr()) };

    if result != 0 {
        return Err(Error::last_os_error().with_ffi("uname"));
    }

    let uts = unsafe { uts.assume_init() };
    let raw_arch = unsafe { CStr::from_ptr(uts.machine.as_ptr()).to_string_lossy() };
    let system = unsafe {
        CStr::from_ptr(uts.sysname.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let release = unsafe {
        CStr::from_ptr(uts.release.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let version = unsafe {
        CStr::from_ptr(uts.version.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let hostname = unsafe {
        CStr::from_ptr(uts.nodename.as_ptr())
            .to_string_lossy()
            .into_owned()
    };

    Ok(Platform {
        system,
        release,
        version,
        hostname,
        arch: Arch::from_str(&raw_arch).unwrap_or(Arch::Unknown),
    })
}
