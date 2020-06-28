use heim_common::units::Information;

/// Linux-specific extension to process [IoCounters] information.
///
/// [IoCounters]: ../../struct.IoCounters.html
pub trait IoCountersExt {
    /// The number of bytes which this task has caused to be read from storage.
    fn chars_read(&self) -> Information;
    /// The number of bytes which this task has caused, or shall cause to be written to disk.
    fn chars_written(&self) -> Information;
    /// Attempt to count the number of read I/O operations,
    /// i.e. syscalls like `read()` and `pread()`.
    fn read_syscalls(&self) -> u64;
    /// Attempt to count the number of write I/O operations,
    /// i.e. syscalls like `write()` and `pwrite()`.
    fn write_syscalls(&self) -> u64;
    // Attempt to count the number of bytes which this process really did cause to
    /// be fetched from the storage layer.
    fn bytes_read(&self) -> Information;
    /// Attempt to count the number of bytes which this process caused to be sent to
    /// the storage layer.
    fn bytes_written(&self) -> Information;
    /// The number of bytes which this process caused to not happen,
    /// by truncating pagecache.
    fn cancelled_write_bytes(&self) -> Information;
}

impl IoCountersExt for crate::process::IoCounters {
    /// The number of bytes which this task has caused to be read from storage.
    fn chars_read(&self) -> Information {
        self.as_ref().chars_read()
    }

    /// The number of bytes which this task has caused, or shall cause to be written to disk.
    fn chars_written(&self) -> Information {
        self.as_ref().chars_written()
    }

    /// Attempt to count the number of read I/O operations,
    /// i.e. syscalls like `read()` and `pread()`.
    fn read_syscalls(&self) -> u64 {
        self.as_ref().read_syscalls()
    }

    /// Attempt to count the number of write I/O operations,
    /// i.e. syscalls like `write()` and `pwrite()`.
    fn write_syscalls(&self) -> u64 {
        self.as_ref().write_syscalls()
    }

    /// Attempt to count the number of bytes which this process really did cause to
    /// be fetched from the storage layer.
    fn bytes_read(&self) -> Information {
        self.as_ref().bytes_read()
    }

    /// Attempt to count the number of bytes which this process caused to be sent to
    /// the storage layer.
    fn bytes_written(&self) -> Information {
        self.as_ref().bytes_written()
    }

    /// The number of bytes which this process caused to not happen,
    /// by truncating pagecache.
    fn cancelled_write_bytes(&self) -> Information {
        self.as_ref().cancelled_write_bytes()
    }
}
