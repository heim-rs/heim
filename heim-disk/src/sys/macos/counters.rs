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

fn filter_map_block_devices(device: iokit::IoObject) -> Result<Option<IoCounters>> {
    let parent = device.parent(b"IOService\0")?;

    if !parent.conforms_to(b"IOBlockStorageDriver\0") {
        return Ok(None);
    }

    let disk_props = device.properties()?;
    let parent_props = parent.properties()?;

    let name = disk_props.get_string("BSD Name")?;
    let stats = parent_props.get_dict("Statistics")?;

    Ok(Some(IoCounters {
        device: name,
        removable: disk_props.get_bool("Removable")?,
        reads: stats.get_i64("Operations (Read)")? as u64,
        writes: stats.get_i64("Operations (Write)")? as u64,
        read_bytes: Information::new::<information::byte>(stats.get_i64("Bytes (Read)")? as u64),
        write_bytes: Information::new::<information::byte>(stats.get_i64("Bytes (Write)")? as u64),
        read_time: Time::new::<time::nanosecond>(stats.get_i64("Total Time (Read)")? as f64),
        write_time: Time::new::<time::nanosecond>(stats.get_i64("Total Time (Write)")? as f64),
    }))
}

pub async fn io_counters() -> Result<impl Stream<Item = Result<IoCounters>>> {
    let port = iokit::IoMasterPort::new()?;

    let iter =
        port.get_services(b"IOMedia\0")?.filter_map(|device| {
            match filter_map_block_devices(device) {
                Ok(Some(counters)) => Some(Ok(counters)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        });

    Ok(stream::iter(iter))
}

pub async fn io_counters_physical() -> Result<impl Stream<Item = Result<IoCounters>>> {
    let inner = io_counters().await?;

    let stream = inner.try_filter(|counter| future::ready(!counter.removable));

    Ok(stream)
}
