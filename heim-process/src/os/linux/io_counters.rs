use std::fmt;

use heim_common::units::{information, Information};

/// Process IO statistics.
///
/// For additional information of data provided, see [proc.txt] documentation,
/// section 3.3 "/proc/<pid>/io - Display the IO accounting fields".
///
/// [proc.txt]: https://www.kernel.org/doc/Documentation/filesystems/proc.txt
#[derive(Default)]
pub struct IoCounters {
    pub(crate) rchar: u64,
    pub(crate) wchar: u64,
    pub(crate) syscr: u64,
    pub(crate) syscw: u64,
    pub(crate) read_bytes: u64,
    pub(crate) write_bytes: u64,
    pub(crate) cancelled_write_bytes: u64,
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
