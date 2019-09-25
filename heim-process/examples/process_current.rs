use heim_common::units::ratio;
use heim_process as process;

#[heim_derive::main]
async fn main() -> Result<(), process::ProcessError> {
    let process = process::current().await?;

    // Let's start with a CPU usage in order to measure how much time it will take
    // to load all the things. See the end of file for a second CPU usage call.
    let cpu_usage = process.cpu_usage().await?;

    dbg!(process.pid());
    dbg!(process.parent().await?);
    dbg!(process.status().await?);
    dbg!(process.name().await?);
    dbg!(process.exe().await?);
    dbg!(process.command().await?);
    #[cfg(not(target_os = "windows"))] // Not implemented yet
    dbg!(process.cwd().await?);
    dbg!(process.create_time().await?);
    dbg!(process.cpu_time().await?);
    dbg!(process.memory().await?);

    #[cfg(target_os = "linux")]
    {
        println!("# Linux specifics");
        use heim_common::prelude::StreamExt;
        use heim_process::os::linux::ProcessExt;

        dbg!(process.io_counters().await?);

        let mut net_io_counters = process.net_io_counters();
        while let Some(counter) = net_io_counters.next().await {
            let counter = counter?;
            dbg!(counter);
        }
    }

    let cpu_usage_2 = process.cpu_usage().await?;
    println!(
        "CPU usage: {} %",
        (cpu_usage_2 - cpu_usage).get::<ratio::percent>()
    );

    Ok(())
}
