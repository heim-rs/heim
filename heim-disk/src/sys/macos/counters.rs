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

async fn get_io_counter(disk: iokit::IoObject) -> Result2<Option<IoCounters>> {
    let parent = unsafe { disk.parent(b"IOService\0") }?;
    let is_block_storage = unsafe { parent.conforms_to(b"IOBlockStorageDriver\0") };
    // It is not a disk, apparently, ignoring it
    if !is_block_storage {
        return Ok(None);
    }

    let disk_props = disk.properties()?;
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

pub fn io_counters() -> impl Stream<Item = Result2<IoCounters>> {
    future::lazy(|_| {
        let port = iokit::IoMasterPort::new()?;

        let services = port.get_services(b"IOMedia\0")?;

        let stream = stream::iter(services).map(Ok);

        Ok(stream)
    })
    .try_flatten_stream()
    .try_filter_map(get_io_counter)
}

pub fn io_counters_physical() -> impl Stream<Item = Result2<IoCounters>> {
    io_counters().try_filter(|counter| future::ready(!counter.removable))
}
