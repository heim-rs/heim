use std::time::{Duration, Instant};
use std::usize;

use heim_common::prelude::{StreamExt, TryStreamExt};
use heim_common::units::{ratio, Ratio};
use heim_process::{self as process, Process, ProcessResult};

async fn usage(process: Process) -> ProcessResult<(process::Process, Ratio)> {
    let usage_1 = process.cpu_usage().await?;
    futures_timer::Delay::new(Duration::from_millis(100)).await;
    let usage_2 = process.cpu_usage().await?;

    Ok((process, usage_2 - usage_1))
}

#[heim_derive::main]
async fn main() -> ProcessResult<()> {
    let start = Instant::now();

    let processes = process::processes()
        .map_ok(|process| {
            // Note that there is no `.await` here,
            // as we want to pass the returned future
            // into the `.try_buffer_unordered`.
            usage(process)
        })
        .try_buffer_unordered(usize::MAX);
    pin_utils::pin_mut!(processes);

    println!("| {:6} | {:40} | {:4} % |", "pid", "name", "CPU");
    while let Some(res) = processes.next().await {
        let (process, usage) = res?;

        println!(
            "| {:6} | {:40} | {:.2} |",
            process.pid(),
            process.name().await?,
            usage.get::<ratio::percent>()
        );
    }

    let end = Instant::now();
    println!(
        "\nIt took {:?} to load and render the processes list",
        (end - start)
    );
    println!(
        "Memory used: {:?}",
        process::current().await?.memory().await?
    );

    Ok(())
}
