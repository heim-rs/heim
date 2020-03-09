use std::io;
use std::error::Error;
use std::process::Command;

use crate::Result;

pub fn free() -> Result<(u64, u64, u64, u64)> {
    let free = Command::new("free")
        .arg("--b")
        .env("LANG", "C.UTF-8")
        .output()?;
    let stdout = String::from_utf8(free.stdout)?;
    for line in stdout.lines() {
        if line.starts_with("Mem:") {
            let mut parts = line.split_whitespace().skip(1).take(4);
            let mut parse = || Ok::<_, Box<dyn Error>>(parts.next()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
                .parse::<u64>()?);

            return Ok((
                parse()?,  // Total
                parse()?,  // Used
                parse()?,  // Free
                parse()?,  // Shared
            ))
        }
    }

    Err(io::Error::from(io::ErrorKind::InvalidData).into())
}
