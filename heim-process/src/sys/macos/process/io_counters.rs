use crate::{ProcessError, ProcessResult};
use heim_common::units::{information, Information};
use heim_common::Pid;
use std::fmt;
use std::io;
pub struct IoCounters {
    read_bytes: u64,
    write_bytes: u64,
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters")
            .field("bytes_read", &self.bytes_read())
            .field("bytes_written", &self.bytes_written())
            .finish()
    }
}

impl IoCounters {
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
}

impl From<darwin_libproc::rusage_info_v2> for IoCounters {
    fn from(info: darwin_libproc::rusage_info_v2) -> IoCounters {
        IoCounters {
            read_bytes: info.ri_diskio_bytesread,
            write_bytes: info.ri_diskio_bytesread,
        }
    }
}

pub async fn io(pid: Pid) -> ProcessResult<IoCounters> {
    match darwin_libproc::pid_rusage::<darwin_libproc::rusage_info_v2>(pid) {
        Ok(contents) => Ok(IoCounters::from(contents)),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            Err(ProcessError::AccessDenied(pid))
        }
        Err(e) => Err(e.into()),
    }
}
