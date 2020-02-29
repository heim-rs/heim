//! Get as much information about process as possible.
//!
//! Process pid should be passed as a program argument,ex.
//!
//! ```
//! $ cargo run --example process -- $$
//! ```
//!
//! `$$` is expanded by bash into its own pid.

use std::env;
use std::error::Error;
use std::io;

#[macro_use]
extern crate prettytable;

use heim::process::{self, Pid};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pid = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?
        .parse::<Pid>()?;

    let process = process::get(pid).await?;
    let table = flip_the_table(process).await?;

    table.print_tty(false);

    Ok(())
}

async fn flip_the_table(p: process::Process) -> process::ProcessResult<prettytable::Table> {
    let mut table = prettytable::Table::new();

    table.add_row(row!["PID", p.pid()]);
    table.add_row(row!["Parent PID", p.parent_pid().await?]);
    table.add_row(row!["Name", p.name().await?]);
    table.add_row(row!["Exe", p.exe().await?.display()]);
    table.add_row(row!["Command", format!("{:?}", p.command().await?)]);
    table.add_row(row!["Environment", format!("{:?}", p.environment().await?)]);
    table.add_row(row!["Status", format!("{:?}", p.status().await?)]);
    table.add_row(row!["Create time", format!("{:?}", p.create_time().await?)]);
    table.add_row(row!["CPU time", format!("{:?}", p.cpu_time().await?)]);

    Ok(table)
}
