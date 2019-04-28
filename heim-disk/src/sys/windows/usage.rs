use std::io;
use std::path::Path;

use winapi::um::{fileapi, winnt};

use heim_common::prelude::*;
use heim_common::units::si::f64::Ratio;
use heim_common::units::si::ratio::ratio;
use heim_common::units::iec::u64::Information;
use heim_common::units::iec::information::byte;

#[derive(Default)]
pub struct Usage {
    total: winnt::ULARGE_INTEGER,
    available: winnt::ULARGE_INTEGER,
    free: winnt::ULARGE_INTEGER,
}

impl Usage {
    pub fn total(&self) -> Information {
        Information::new::<byte>(unsafe {
            *self.total.QuadPart()
        })
    }

    pub fn used(&self) -> Information {
        self.total() - self.free()
    }

    pub fn free(&self) -> Information {
        Information::new::<byte>(unsafe {
            *self.total.QuadPart()
        })
    }

    pub fn ratio(&self) -> Ratio {
        // TODO: IEC system should declare it's own Ratio?
        // TODO: Possible value truncation
        Ratio::new::<ratio>(self.used().value as f64 / self.total().value as f64)
    }
}

pub fn usage<T: AsRef<Path>>(path: T) -> impl Future<Output=Result<Usage>> {
    future::lazy(move |_| {
        let path = match widestring::U16CString::from_os_str(path.as_ref()) {
            Ok(path) => path,
            Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidInput).into())
        };

        let mut usage = Usage::default();
        let result = unsafe {
            fileapi::GetDiskFreeSpaceExW(
                path.as_ptr(),
                &mut usage.available as &mut _ as winnt::PULARGE_INTEGER,
                &mut usage.total as &mut _ as winnt::PULARGE_INTEGER,
                &mut usage.free as &mut _ as winnt::PULARGE_INTEGER,
            )
        };

        if result != 0 {
            Ok(usage)
        } else {
            Err(Error::last_os_error())
        }
    })
}
