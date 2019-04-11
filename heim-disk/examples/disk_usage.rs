use heim_common::prelude::*;
use heim_common::units::iec::information::megabyte;
use heim_disk as disk;
/// Command similar to `df -BM`
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut runtime = runtime::new()?;

    println!(
        "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
        "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type", "Mount",
    );

    for part in runtime.block_collect(disk::partitions_physical()) {
        let part = part?;
        let usage = runtime.block_run(disk::usage(part.mount_point().to_path_buf()))?;

        println!(
            "{:<17} {:<10} {:<10} {:<10} {:<10?} {}",
            part.device().unwrap_or("N/A"),
            usage.total().get::<megabyte>(),
            usage.used().get::<megabyte>(),
            usage.free().get::<megabyte>(),
            part.file_system(),
            part.mount_point().to_string_lossy(),
        );
    }

    Ok(())
}
