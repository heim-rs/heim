use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::units::{information, Information};
use heim_common::utils::iter::TryIterator;
use heim_common::Pid;
use heim_runtime as rt;
use std::fmt;

use crate::sys::linux::process::procfs::process_file_path;
use crate::{ProcessError, ProcessResult};

/// Process IO statistics.
///
/// For additional information of data provided, see [proc.txt] documentation,
/// section 3.3 "/proc/<pid>/io - Display the IO accounting fields".
///
/// [proc.txt]: https://www.kernel.org/doc/Documentation/filesystems/proc.txt
#[derive(Default)]
pub struct IoCounters {
    rchar: u64,
    wchar: u64,
    syscr: u64,
    syscw: u64,
    read_bytes: u64,
    write_bytes: u64,
    cancelled_write_bytes: u64,
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("chars_read", &self.chars_read())
            .field("chars_written", &self.chars_written())
            .field("read_syscalls", &self.read_syscalls())
            .field("write_syscalls", &self.write_syscalls())
            .field("bytes_read", &self.bytes_read())
            .field("bytes_written", &self.bytes_written())
            .field("cancelled_write_bytes", &self.cancelled_write_bytes())
            .finish()
    }
}

impl IoCounters {
    /// The number of bytes which this task has caused to be read from storage.
    pub fn chars_read(&self) -> Information {
        Information::new::<information::byte>(self.rchar)
    }

    /// The number of bytes which this task has caused, or shall cause to be written to disk.
    pub fn chars_written(&self) -> Information {
        Information::new::<information::byte>(self.wchar)
    }

    /// Attempt to count the number of read I/O operations,
    /// i.e. syscalls like `read()` and `pread()`.
    pub fn read_syscalls(&self) -> u64 {
        self.syscr
    }

    /// Attempt to count the number of write I/O operations,
    /// i.e. syscalls like `write()` and `pwrite()`.
    pub fn write_syscalls(&self) -> u64 {
        self.syscw
    }

    /// Attempt to count the number of bytes which this process really did cause to
    /// be fetched from the storage layer.
    pub fn bytes_read(&self) -> Information {
        Information::new::<information::byte>(self.read_bytes)
    }

    /// Attempt to count the number of bytes which this process caused to be sent to
    /// the storage layer.
    pub fn bytes_written(&self) -> Information {
        Information::new::<information::byte>(self.write_bytes)
    }

    /// The number of bytes which this process caused to not happen,
    /// by truncating pagecache.
    pub fn cancelled_write_bytes(&self) -> Information {
        Information::new::<information::byte>(self.cancelled_write_bytes)
    }
}

impl FromStr for IoCounters {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut counters = IoCounters::default();
        for line in s.lines() {
            let mut parts = line.split_ascii_whitespace();
            let field = match parts.try_next()? {
                "rchar:" => &mut counters.rchar,
                "wchar:" => &mut counters.wchar,
                "syscr:" => &mut counters.syscr,
                "syscw:" => &mut counters.syscw,
                "read_bytes:" => &mut counters.read_bytes,
                "write_bytes:" => &mut counters.write_bytes,
                "cancelled_write_bytes:" => &mut counters.cancelled_write_bytes,
                _ => continue,
            };

            *field = parts.try_next()?.parse::<u64>()?;
        }

        Ok(counters)
    }
}

pub async fn io(pid: Pid) -> ProcessResult<IoCounters> {
    let path = process_file_path(pid, "io");
    match rt::fs::read_to_string(path).await {
        Ok(contents) => IoCounters::from_str(&contents).map_err(Into::into),
        Err(e) if e.raw_os_error() == Some(libc::EACCES) => Err(ProcessError::AccessDenied(pid)),
        Err(e) => Err(e.into()),
    }
}
