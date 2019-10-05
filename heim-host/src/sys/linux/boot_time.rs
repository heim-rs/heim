use std::io;

use heim_common::prelude::*;
use heim_common::units::{time, Time};
use heim_runtime::fs;

pub fn parse(stat: &str) -> Result2<Time> {
    for line in stat.lines() {
        if line.starts_with("btime ") {
            let mut parts = line.splitn(2, ' ');
            let _ = parts.next();

            match parts.next() {
                Some(raw_value) => {
                    let seconds = raw_value.parse::<f64>()?;

                    return Ok(Time::new::<time::second>(seconds));
                }
                None => break,
            }
        }
    }

    Err(Error2::from(io::Error::from(io::ErrorKind::InvalidData))
        .with_message("Unable to find `btime` value"))
}

pub async fn boot_time() -> Result2<Time> {
    let stat = fs::read_to_string("/proc/stat").await?;

    parse(&stat)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[heim_derive::test]
    async fn test_stat_parse() {
        let result = parse(&PROC_STAT);
        assert!(result.is_ok());
    }

    const PROC_STAT: &str = include_str!("../../../assets/linux_proc_stat.txt");
}
