//! Get as much information about process as possible.
//!
//! Process pid should be passed as a program argument,ex.
//!
//! ```
//! $ cargo run --example process -- $$
//! ```
//!
//! `$$` is expanded by bash into its own pid.

#[cfg(not(target_os = "windows"))]
use std::collections::HashMap;
use std::env;
use std::error::Error;

#[macro_use]
extern crate prettytable;

use heim::process;

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        let process = match env::args().nth(1) {
            Some(value) => {
                let pid = value.parse()?;
                process::get(pid).await?
            }
            None => {
                eprintln!(
                    "Process PID is not passed as an argument, self PID will be used instead"
                );
                process::current().await?
            }
        };
        let table = flip_the_table(process).await?;

        table.print_tty(false);

        Ok(())
    })
}

async fn flip_the_table(p: process::Process) -> process::ProcessResult<prettytable::Table> {
    let mut table = prettytable::Table::new();

    table.add_row(row!["PID", p.pid()]);
    table.add_row(row!["Parent PID", p.parent_pid().await?]);
    table.add_row(row!["Name", p.name().await?]);
    table.add_row(row!["Exe", p.exe().await?.display()]);
    #[cfg(not(target_os = "windows"))] // Not implemented yet
    table.add_row(row!["Command", format!("{:?}", p.command().await?)]);
    #[cfg(not(target_os = "windows"))] // Not implemented yet
    table.add_row(row!["Current working dir", format!("{:?}", p.cwd().await?)]);
    #[cfg(not(target_os = "windows"))] // Not implemented yet
    table.add_row(row![
        "Environment",
        format!(
            "{:?}, ..",
            p.environment()
                .await?
                .iter()
                .take(3)
                .collect::<HashMap::<_, _>>()
        )
    ]);
    table.add_row(row!["Status", format!("{:?}", p.status().await?)]);
    table.add_row(row!["Create time", format!("{:?}", p.create_time().await?)]);
    table.add_row(row!["CPU time", format!("{:?}", p.cpu_time().await?)]);

    Ok(table)
}
