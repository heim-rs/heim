use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::units::{information, Information};
use heim_runtime::fs;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, heim_derive::Getter)]
pub struct Memory {
    total: Information,     // MemTotal
    free: Information,      // MemFree
    available: Information, // MemAvailable
    buffers: Information,   // Buffers
    cached: Information,    // Cached
    active: Information,    // Active
    inactive: Information,  // Inactive
    shared: Information,    // Shmem
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
                b"Me" | b"Ac" | b"In" | b"Bu" | b"Ca" | b"Sh" => {}
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

        // TODO: Kinda bad to name it like this
        Err(Error::missing_entity("<unknown>"))
    }
}

pub fn memory() -> impl Future<Output = Result<Memory>> {
    fs::read_into("/proc/meminfo")
}
