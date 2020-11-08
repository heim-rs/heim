use std::error::Error;
use std::time::Duration;

#[macro_use]
extern crate prettytable;
use futures::{StreamExt, TryStreamExt};

use heim::process::{self, Memory, Process, ProcessResult};
use heim::units::{information, Ratio};

async fn usage(process: Process) -> ProcessResult<(Process, Ratio, Memory)> {
    let usage_1 = process.cpu_usage().await?;
    futures_timer::Delay::new(Duration::from_millis(100)).await;
    // TODO: Can use `.join()` here
    let usage_2 = process.cpu_usage().await?;
    let memory = process.memory().await?;

    Ok((process, usage_2 - usage_1, memory))
}

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        let mut table = prettytable::Table::new();
        table.set_titles(row![
            "PID",
            "Name",
            "Status",
            "CPU",
            "Memory",
            "Virtual memory"
        ]);

        let processes = process::processes()
            .await?
            .map_ok(|process| {
                // Note that there is no `.await` here,
                // as we want to pass the returned future
                // into the `.try_buffer_unordered`.
                usage(process)
            })
            .try_buffer_unordered(usize::MAX);

        futures::pin_mut!(processes);

        while let Some(res) = processes.next().await {
            if let Ok((process, usage, memory)) = res {
                table.add_row(row![
                    format!("{}", process.pid()),
                    process.name().await?,
                    format!("{:?}", process.status().await?),
                    format!("{:?}", usage),
                    format!("{:?} KB", memory.rss().get::<information::kilobyte>()),
                    format!("{:?} KB", memory.vms().get::<information::kilobyte>()),
                ]);
            }
        }

        table.print_tty(false);

        Ok(())
    })
}
//
//async fn flip_the_table(p: process::Process) -> process::ProcessResult<prettytable::Table> {
//    let mut table = prettytable::Table::new();
//
//    table.add_row(row!["PID", p.pid()]);
//    table.add_row(row!["Parent PID", p.parent_pid().await?]);
//    table.add_row(row!["Name", p.name().await?]);
//    table.add_row(row!["Exe", p.exe().await?.display()]);
//    #[cfg(not(target_os = "windows"))] // Not implemented yet
//    table.add_row(row!["Command", format!("{:?}", p.command().await?)]);
//    #[cfg(not(target_os = "windows"))] // Not implemented yet
//    table.add_row(row!["Current working dir", format!("{:?}", p.cwd().await?)]);
//    #[cfg(not(target_os = "windows"))] // Not implemented yet
//    table.add_row(row![
//        "Environment",
//        format!(
//            "{:?}, ..",
//            p.environment()
//                .await?
//                .iter()
//                .take(3)
//                .collect::<HashMap::<_, _>>()
//        )
//    ]);
//    table.add_row(row!["Status", format!("{:?}", p.status().await?)]);
//    table.add_row(row!["Create time", format!("{:?}", p.create_time().await?)]);
//    table.add_row(row!["CPU time", format!("{:?}", p.cpu_time().await?)]);
//
//    Ok(table)
//}
