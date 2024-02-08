use std::io;
use std::str::FromStr;

use heim_common::prelude::{Error, Result};
use heim_common::units::{information, Information};
use heim_runtime as rt;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Memory {
    total: Information,     // MemTotal
    free: Information,      // MemFree
    available: Information, // MemAvailable
    buffers: Information,   // Buffers
    cached: Information,    // Cached
    active: Information,    // Active
    inactive: Information,  // Inactive
    shared: Information,    // Shmem
    dirty: Information,     // Dirty
}

impl Memory {
    pub fn total(&self) -> Information {
        self.total
    }
    pub fn free(&self) -> Information {
        self.free
    }
    pub fn available(&self) -> Information {
        self.available
    }
    pub fn buffers(&self) -> Information {
        self.buffers
    }
    pub fn cached(&self) -> Information {
        self.cached
    }
    pub fn active(&self) -> Information {
        self.active
    }
    pub fn inactive(&self) -> Information {
        self.inactive
    }
    pub fn shared(&self) -> Information {
        self.shared
    }

    pub fn dirty(&self) -> Information {
        self.dirty
    }
}

impl FromStr for Memory {
    type Err = Error;

    fn from_str(meminfo: &str) -> Result<Self> {
        let mut memory = Memory::default();
        let mut matched_lines = 0u8;

        for line in meminfo.lines() {
            // If line does not starts with "Me", "Ac" or other options at all,
            // we do not need that key at all
            let first_bytes = &line.as_bytes()[..2];
            match first_bytes {
                b"Me" | b"Ac" | b"In" | b"Bu" | b"Ca" | b"Sh" | b"Di" => {}
                _ => continue,
            };

            let mut parts = line.splitn(2, ':');
            let field = match parts.next() {
                Some("MemTotal") => &mut memory.total,
                Some("MemFree") => &mut memory.free,
                Some("MemAvailable") => &mut memory.available,
                Some("Buffers") => &mut memory.buffers,
                Some("Cached") => &mut memory.cached,
                Some("Active") => &mut memory.active,
                Some("Inactive") => &mut memory.inactive,
                Some("Shmem") => &mut memory.shared,
                Some("Dirty") => &mut memory.dirty,
                _ => continue,
            };

            match parts.next() {
                Some(value) => {
                    *field = {
                        let bytes = match value.trim_start().splitn(2, ' ').next() {
                            Some(kbytes) => {
                                let value = kbytes.parse::<u64>()?;
                                Information::new::<information::kilobyte>(value)
                            }
                            None => continue,
                        };

                        matched_lines += 1;

                        bytes
                    }
                }
                None => continue,
            }

            if matched_lines == 8 {
                return Ok(memory);
            }
        }

        // `FromStr` knows nothing about `/proc/meminfo`,
        // but at this point we are not tracking which exact field are we missing.
        // TODO: Rewrite parser and use `Error::missing_key` instead
        let inner = io::Error::from(io::ErrorKind::InvalidData);
        Err(Error::from(inner).with_file(rt::linux::procfs_root().join("meminfo")))
    }
}

pub async fn memory() -> Result<Memory> {
    rt::fs::read_into(rt::linux::procfs_root().join("meminfo")).await
}
