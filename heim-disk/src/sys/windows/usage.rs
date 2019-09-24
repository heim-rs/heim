use std::fmt;
use std::io;
use std::path::Path;

use winapi::um::{fileapi, winnt};

use heim_common::prelude::*;
use heim_common::units::{information, ratio, Information, Ratio};

#[derive(Default)]
pub struct Usage {
    total: winnt::ULARGE_INTEGER,
    available: winnt::ULARGE_INTEGER,
    free: winnt::ULARGE_INTEGER,
}

impl Usage {
    pub fn total(&self) -> Information {
        Information::new::<information::byte>(unsafe { *self.total.QuadPart() })
    }

    pub fn used(&self) -> Information {
        self.total() - self.free()
    }

    pub fn free(&self) -> Information {
        Information::new::<information::byte>(unsafe { *self.free.QuadPart() })
    }

    pub fn ratio(&self) -> Ratio {
        // TODO: Possible value truncation
        Ratio::new::<ratio::ratio>(
            (self.used().get::<information::byte>() as f64
                / self.total().get::<information::byte>() as f64) as f32,
        )
    }
}

impl fmt::Debug for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Usage")
            .field("total", &self.total())
            .field("used", &self.used())
            .field("free", &self.free())
            .field("ratio", &self.ratio())
            .finish()
    }
}

pub fn usage<T: AsRef<Path>>(path: T) -> impl Future<Output = Result<Usage>> {
    future::lazy(move |_| {
        let path = match widestring::U16CString::from_os_str(path.as_ref()) {
            Ok(path) => path,
            Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidInput).into()),
        };

        let mut usage = Usage::default();
        let result = unsafe {
            fileapi::GetDiskFreeSpaceExW(
                path.as_ptr(),
                &mut usage.available,
                &mut usage.total,
                &mut usage.free,
            )
        };

        if result != 0 {
            Ok(usage)
        } else {
            Err(Error::last_os_error())
        }
    })
}
