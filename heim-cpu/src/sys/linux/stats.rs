use std::str::FromStr;

use heim_common::prelude::*;

#[derive(Debug, Default, heim_derive::Getter)]
pub struct CpuStats {
    ctx_switches: u64,
    interrupts: u64,
    soft_interrupts: u64,
}

impl FromStr for CpuStats {
    type Err = Error;

    fn from_str(s: &str) -> Result<CpuStats> {
        let mut stats = CpuStats::default();
        let mut matched_lines = 0u8;

        for line in s.lines() {
            let mut parts = line.split(' ');
            let (name, field) = match parts.next() {
                Some(name) if name == "ctxt" => ("ctxt", &mut stats.ctx_switches),
                Some(name) if name == "intr" => ("intr", &mut stats.interrupts),
                Some(name) if name == "softirq" => ("softirq", &mut stats.soft_interrupts),
                _ => continue,
            };

            match parts.next() {
                Some(raw_value) => {
                    let value = raw_value.trim_end().parse::<u64>()?;
                    matched_lines += 1;
                    *field = value;
                }
                None => return Err(Error::missing_entity(name)),
            }

            if matched_lines == 3 {
                break;
            }
        }

        Ok(stats)
    }
}

pub fn stats() -> impl Future<Output = Result<CpuStats>> {
    utils::fs::read_into("/proc/stat")
}
