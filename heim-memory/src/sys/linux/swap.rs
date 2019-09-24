use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::units::{information, Information};
use heim_runtime::fs;

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

        Err(Error::missing_entity("<unknown>"))
    }
}

fn vm_stat() -> impl Future<Output = Result<VmStat>> {
    fs::read_into(PROC_VMSTAT)
}

pub fn swap() -> impl Future<Output = Result<Swap>> {
    let meminfo = fs::read_to_string(PROC_MEMINFO);
    // TODO: Replace with `try_join`
    future::join(meminfo, vm_stat()).then(|result| match result {
        (Ok(string), Ok(vm_stat)) => future::ready(Swap::parse_str(&string, vm_stat)),
        (Err(e), _) => future::err(e.into()),
        (_, Err(e)) => future::err(e),
    })
}
