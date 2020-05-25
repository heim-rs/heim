use std::fs;
use std::io;
use std::str::FromStr;

use heim_runtime as rt;

use heim_common::prelude::*;
use heim_common::units::{information, Information};

static PROC_VMSTAT: &str = "/proc/vmstat";
static PROC_MEMINFO: &str = "/proc/meminfo";

#[derive(Debug, Default, Clone)]
pub struct VmStat {
    swap_in: Option<Information>,  // pswpin
    swap_out: Option<Information>, // pswpout
}

impl FromStr for VmStat {
    type Err = Error;

    fn from_str(vmstat: &str) -> Result<Self> {
        let mut stat = VmStat::default();

        for line in vmstat.lines() {
            let first_bytes = &line.as_bytes()[..2];
            if first_bytes != b"ps" {
                continue;
            }

            let mut parts = line.splitn(2, ' ');
            let field = match parts.next() {
                Some("pswpin") => &mut stat.swap_in,
                Some("pswpout") => &mut stat.swap_out,
                _ => continue,
            };

            match parts.next() {
                Some(value) => {
                    *field = {
                        let bytes = match value.trim_start().splitn(2, ' ').next() {
                            Some(kbytes) => {
                                // Values are expressed in 4 kilo bytes, we want bytes instead.
                                // Source: psutil
                                let value = kbytes.parse::<u64>()?;
                                Information::new::<information::kilobyte>(4 * value)
                            }
                            None => continue,
                        };

                        Some(bytes)
                    }
                }
                None => continue,
            }
        }

        Ok(stat)
    }
}

#[derive(Debug, Clone)]
pub struct Swap {
    total: Information, // SwapTotal
    free: Information,  // SwapFree
    vm_stat: VmStat,
}

impl Swap {
    pub fn total(&self) -> Information {
        self.total
    }

    pub fn used(&self) -> Information {
        self.total - self.free
    }

    pub fn free(&self) -> Information {
        self.free
    }

    pub fn sin(&self) -> Option<Information> {
        self.vm_stat.swap_in
    }

    pub fn sout(&self) -> Option<Information> {
        self.vm_stat.swap_out
    }

    pub fn parse_str(meminfo: &str, vm_stat: VmStat) -> Result<Self> {
        let mut swap = Swap {
            total: Information::new::<information::byte>(0),
            free: Information::new::<information::byte>(0),
            vm_stat,
        };
        let mut matched_lines = 0u8;

        for line in meminfo.lines() {
            // If line does not starts with "Sw" we do not need that key at all
            let first_bytes = &line.as_bytes()[..2];
            if first_bytes != b"Sw" {
                continue;
            }

            let mut parts = line.splitn(2, ':');
            let field = match parts.next() {
                Some("SwapTotal") => &mut swap.total,
                Some("SwapFree") => &mut swap.free,
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

            if matched_lines == 2 {
                return Ok(swap);
            }
        }

        // `FromStr` knows nothing about `/proc/meminfo`,
        // but at this point we are not tracking which exact field are we missing.
        // TODO: Rewrite parser and use `Error::missing_key` instead
        let inner = io::Error::from(io::ErrorKind::InvalidData);
        Err(Error::from(inner).with_file(PROC_MEMINFO))
    }
}

pub async fn swap() -> Result<Swap> {
    rt::spawn_blocking(|| {
        let meminfo = fs::read_to_string(PROC_MEMINFO)?;
        let vmstat = fs::read_to_string(PROC_VMSTAT)?;
        let vmstat = VmStat::from_str(&vmstat)?;

        Swap::parse_str(&meminfo, vmstat)
    }).await
}
