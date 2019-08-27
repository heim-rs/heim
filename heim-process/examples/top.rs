use std::time::Duration;

use futures::stream::{StreamExt, TryStreamExt};
use heim_common::units::Ratio;
use heim_process::{self as process, Process, ProcessResult};

async fn usage(process: Process) -> ProcessResult<(process::Process, Ratio)> {
    let usage_1 = process.cpu_usage().await?;
    futures_timer::Delay::new(Duration::from_millis(100)).await?;
    let usage_2 = process.cpu_usage().await?;

    Ok((process, usage_2 - usage_1))
}

#[heim_derive::main]
async fn main() -> ProcessResult<()> {
    let processes = process::processes().and_then(usage);
    pin_utils::pin_mut!(processes);

    // Note that this example is resolving processes one by one,
    // and `futures_timer::Delay`, well, delays execution for each process yielded,
    // therefore output will be slow. Consider using buffering or other tricks
    // to calculate CPU usage for multiple processes at once.

    println!("| {:6} | {:40} | {:4} % |", "pid", "name", "CPU");
    while let Some(res) = processes.next().await {
        let (process, usage) = res?;

        println!(
            "| {:6} | {:40} | {:.2} |",
            process.pid(),
            process.name().await?,
            usage.get() * 100.0
        );
    }

    Ok(())
}
