use heim_common::units::{information, Information};
use std::fmt;
use winapi::um::winnt;
/// Process IO statistics.
/// For additional information, see [IO_COUNTERS] documentation.
///
/// [IO_COUNTERS]: https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-io_counters
pub struct IoCounters(winnt::IO_COUNTERS);

impl IoCounters {
    /// The number of read operations performed.
    pub fn read_iops(&self) -> u64 {
        self.0.ReadOperationCount
    }

    /// The number of read operations performed.
    pub fn write_iops(&self) -> u64 {
        self.0.WriteOperationCount
    }

    /// The number of I/O operations performed, other than read and write operations.
    pub fn other_iops(&self) -> u64 {
        self.0.OtherOperationCount
    }

    /// The number of bytes read.
    pub fn bytes_read(&self) -> Information {
        Information::new::<information::byte>(self.0.ReadTransferCount)
    }

    /// The number of bytes written.
    pub fn bytes_written(&self) -> Information {
        Information::new::<information::byte>(self.0.WriteTransferCount)
    }

    /// The number of bytes transferred during operations other than read and write operations.
    pub fn bytes_other(&self) -> Information {
        Information::new::<information::byte>(self.0.OtherTransferCount)
    }
}

impl From<winnt::IO_COUNTERS> for IoCounters {
    fn from(info: winnt::IO_COUNTERS) -> IoCounters {
        IoCounters(info)
    }
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("bytes_read", &self.bytes_read())
            .field("bytes_written", &self.bytes_written())
            .field("bytes_other", &self.bytes_other())
            .field("read_iops", &self.read_iops())
            .field("write_iops", &self.write_iops())
            .field("other_iops", &self.other_iops())
            .finish()
    }
}
