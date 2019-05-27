use std::io;
use std::mem;
use std::ffi::CString;
use std::path::Path;

use heim_common::prelude::*;
use heim_common::units::{Information, Ratio};

use crate::os::unix::Flags;

pub struct Usage(libc::statvfs);

// Why there are `u64::from()` everywhere -- to mitigate the differences
// between `libc::statvfs` for x86 and `libc::statvfs` for x86_64,
// fields can be either `u32` or `u64`.
#[allow(clippy::identity_conversion)]
impl Usage {
    pub fn total(&self) -> Information {
        let value = u64::from(self.0.f_blocks) * u64::from(self.0.f_frsize);

        Information::new(value)
    }

    pub fn used(&self) -> Information {
        let avail_to_root = u64::from(self.0.f_bfree) * u64::from(self.0.f_frsize);

        self.total() - Information::new(avail_to_root)
    }

    pub fn free(&self) -> Information {
        let value = u64::from(self.0.f_bavail) * u64::from(self.0.f_frsize);

        Information::new(value)
    }

    pub fn ratio(&self) -> Ratio {
        // FIXME: Possible value truncation while casting into f64.
        // Lucky us, it is a 2019 and we are good for the next couple of decades
        let used = *self.used().as_ref() as f32;
        let avail_to_user = u64::from(self.0.f_bavail) * u64::from(self.0.f_frsize);
        let total_user = used + avail_to_user as f32;

        Ratio::new(used / total_user)
    }

    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.0.f_flag)
    }
}

pub fn usage<T: AsRef<Path>>(path: T) -> impl Future<Output=Result<Usage>> {
    future::lazy(move |_| {
        let path = path.as_ref().to_str()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
            .and_then(|string| {
                CString::new(string).map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))
            })?;

        let mut vfs = mem::MaybeUninit::<libc::statvfs>::uninit();
        let result = unsafe {
            libc::statvfs(path.into_raw(), vfs.as_mut_ptr())
        };

        if result == 0 {
            let vfs = unsafe {
                vfs.assume_init()
            };
            Ok(Usage(vfs))
        } else {
            Err(Error::last_os_error())
        }
    })
}
