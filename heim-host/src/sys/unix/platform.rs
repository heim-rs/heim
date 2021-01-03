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
    unsafe {
        let mut uts = mem::MaybeUninit::<libc::utsname>::uninit();
        let result = libc::uname(uts.as_mut_ptr());

        if result != 0 {
            Err(Error::last_os_error().with_ffi("uname"))
        } else {
            let uts = uts.assume_init();
            let raw_arch = CStr::from_ptr(uts.machine.as_ptr()).to_string_lossy();
            let arch = arch_from_uname(&raw_arch).unwrap_or_else(|| {
                log::error!("Unable to parse CPU architecture from \"{}\"", raw_arch);
                Arch::Unknown
            });

            Ok(Platform {
                system: CStr::from_ptr(uts.sysname.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                release: CStr::from_ptr(uts.release.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                version: CStr::from_ptr(uts.version.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                hostname: CStr::from_ptr(uts.nodename.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                arch,
            })
        }
    }
}

/// Arch::from_str only handles the exact names used
/// by the rust compiler (that's the policy of the platforms crate)
/// However, uname -m has more varied output, which we need to detect
fn arch_from_uname(raw: &str) -> Option<Arch> {
    if let Ok(arch) = Arch::from_str(raw) {
        return Some(arch);
    };

    match raw {
        "armv7" | "armv7l" | "arm64" => Some(Arch::ARM),
        "ppc64" | "ppc64le" => Some(Arch::POWERPC64),
        _ => None,
    }
}
