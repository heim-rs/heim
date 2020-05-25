//! Naive clone of the `pidof` utility

use std::env;
use std::ffi::OsStr;
use std::io;

use futures::StreamExt as _;

use heim::{
    process::{self, Process, ProcessResult},
    Result,
};

async fn compare(process: ProcessResult<Process>, needle: &str) -> Option<process::Pid> {
    let process = process.ok()?;
    if needle == process.name().await.ok()? {
        return Some(process.pid());
    }

    let command = process.command().await.ok()?;
    if Some(&OsStr::new(needle)) == command.into_iter().next().as_ref() {
        return Some(process.pid());
    }

    None
}

#[smol_potat::main]
async fn main() -> Result<()> {
    let needle = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Program name argument is missing",
            )
            .into())
        }
    };

    let processes = process::processes().await?;
    futures::pin_mut!(processes);
    while let Some(process) = processes.next().await {
        if let Some(pid) = compare(process, &needle).await {
            print!("{} ", pid);
        }
    }

    println!();

    Ok(())
}
