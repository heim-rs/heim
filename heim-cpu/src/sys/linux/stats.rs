use std::io;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_runtime::fs;

#[derive(Debug, Default, heim_derive::Getter)]
pub struct CpuStats {
    ctx_switches: u64,
    interrupts: u64,
    soft_interrupts: u64,
}

impl FromStr for CpuStats {
    type Err = Error2;

    fn from_str(s: &str) -> Result2<CpuStats> {
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
                None => {
                    let e = Error2::from(io::Error::from(io::ErrorKind::InvalidData))
                        .with_message(format!("Field {} has no value", name));

                    return Err(e);
                }
            }

            if matched_lines == 3 {
                break;
            }
        }

        Ok(stats)
    }
}

pub async fn stats() -> Result2<CpuStats> {
    fs::read_into("/proc/stat").map_err(Into::into).await
}
