use std::ffi::OsStr;

use heim_common::prelude::*;
use heim_common::sys::macos::iokit::{self, DictionaryProps};
use heim_common::units::{information, time, Information, Time};

#[derive(Debug)]
pub struct IoCounters {
    device: String,
    removable: bool,
    reads: u64,
    writes: u64,
    read_bytes: Information,
    write_bytes: Information,
    read_time: Time,
    write_time: Time,
}

impl IoCounters {
    pub fn device_name(&self) -> &OsStr {
        OsStr::new(self.device.as_str())
    }

    pub fn read_count(&self) -> u64 {
        self.reads
    }

    pub fn write_count(&self) -> u64 {
        self.writes
    }

    pub fn read_bytes(&self) -> Information {
        self.read_bytes
    }

    pub fn write_bytes(&self) -> Information {
        self.write_bytes
    }

    pub fn read_time(&self) -> Time {
        self.read_time
    }

    pub fn write_time(&self) -> Time {
        self.write_time
    }
}

pub fn io_counters() -> impl Stream<Item = Result<IoCounters>> {
    future::lazy(|_| {
        let port = iokit::IoMasterPort::new()?;

        let services = port.get_services(b"IOMedia\0")?;

        let stream = stream::iter(services).map(Ok);

        Ok(stream)
    })
    .try_flatten_stream()
    .map(|io_object: Result<iokit::IoObject>| {
        match io_object {
            Ok(obj) => {
                let parent = obj.parent(b"IOService\0")?;
                Ok((obj, parent))
            },
            Err(e) => Err(e),
        }
    })
    .try_filter(|(_disk, parent)| {
        future::ready(parent.conforms_to(b"IOBlockStorageDriver\0"))
    })
    .map(|result| {
        match result {
            Ok((disk, parent)) => {
                let disk_props = disk.properties()?;
                let parent_props = parent.properties()?;

                let name = disk_props.get_string("BSD Name")?;
                let stats = parent_props.get_dict("Statistics")?;

                Ok(IoCounters{
                    device: name,
                    removable: disk_props.get_bool("Removable")?,
                    reads: stats.get_i64("Operations (Read)")? as u64,
                    writes: stats.get_i64("Operations (Write)")? as u64,
                    read_bytes: Information::new::<information::byte>(stats.get_i64("Bytes (Read)")? as u64),
                    write_bytes: Information::new::<information::byte>(stats.get_i64("Bytes (Write)")? as u64),
                    read_time: Time::new::<time::nanosecond>(stats.get_i64("Total Time (Read)")? as f64),
                    write_time: Time::new::<time::nanosecond>(stats.get_i64("Total Time (Write)")? as f64),
                })
            },
            Err(e) => {
                Err(e)
            },
        }
    })
}

pub fn io_counters_physical() -> impl Stream<Item = Result<IoCounters>> {
    io_counters().try_filter(|counter| future::ready(!counter.removable))
}
