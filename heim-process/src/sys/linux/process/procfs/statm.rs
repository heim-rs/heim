use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::units::{information, Information};
use heim_common::utils::iter::ParseIterator;
use heim_runtime::fs;

use crate::{Pid, ProcessResult};

// TODO: Join with `heim-memory/src/sys/macos/mod.rs:PAGE_SIZE`
lazy_static::lazy_static! {
    static ref PAGE_SIZE: u64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as u64
        }
    };
}

#[derive(Debug)]
pub struct Memory {
    size: Information,
    resident: Information,
    shared: Information,
    text: Information,
    data: Information,
}

impl Memory {
    pub fn rss(&self) -> Information {
        self.resident
    }

    pub fn vms(&self) -> Information {
        self.size
    }

    pub fn shared(&self) -> Information {
        self.shared
    }

    pub fn text(&self) -> Information {
        self.text
    }

    pub fn data(&self) -> Information {
        self.data
    }
}

impl FromStr for Memory {
    type Err = Error;

    fn from_str(value: &str) -> Result<Memory> {
        let mut parts = value.split_ascii_whitespace();
        let size = parts
            .try_parse_next::<u64, _>()
            .map(|value| Information::new::<information::byte>(value * *PAGE_SIZE))?;
        let resident = parts
            .try_parse_next::<u64, _>()
            .map(|value| Information::new::<information::byte>(value * *PAGE_SIZE))?;
        let shared = parts
            .try_parse_next::<u64, _>()
            .map(|value| Information::new::<information::byte>(value * *PAGE_SIZE))?;
        let text = parts
            .try_parse_next::<u64, _>()
            .map(|value| Information::new::<information::byte>(value * *PAGE_SIZE))?;
        let _lib = parts.next();
        let data = parts
            .try_parse_next::<u64, _>()
            .map(|value| Information::new::<information::byte>(value * *PAGE_SIZE))?;

        Ok(Memory {
            size,
            resident,
            shared,
            text,
            data,
        })
    }
}

pub fn stat_memory(pid: Pid) -> impl Future<Output = ProcessResult<Memory>> {
    fs::read_into(format!("/proc/{}/statm", pid)).map_err(Into::into)
}
