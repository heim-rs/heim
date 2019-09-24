//! Sending signals to processes
//!
//! ## Usage
//!
//! `$ signal {SIGNAL_TYPE} {PID}`
//!
//! where `{SIGNAL_TYPE}` is one of `suspend`, `resume`, `terminate` or `kill`.
//!
//! Ex. `$ signal terminate 12345`
//! or in a full form:
//! `$ cargo run -p heim-process --example signal -- resume 37520 `

use std::env;
use std::io;

use heim_process as process;

#[heim_derive::main]
async fn main() -> process::ProcessResult<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err(io::Error::from(io::ErrorKind::InvalidData).into());
    }
    let pid: process::Pid = args[2]
        .parse::<process::Pid>()
        .map_err(|_| process::ProcessError::from(io::Error::from(io::ErrorKind::InvalidData)))?;
    let process = process::get(pid).await?;

    match args[1].as_str() {
        "suspend" => process.suspend().await,
        "resume" => process.resume().await,
        "terminate" => process.terminate().await,
        "kill" => process.kill().await,
        _ => Err(io::Error::from(io::ErrorKind::InvalidData).into()),
    }
}
