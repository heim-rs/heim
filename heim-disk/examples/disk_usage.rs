#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_common::units::iec::information::megabyte;
use heim_disk as disk;
/// Command similar to `df -BM`

#[runtime::main]
async fn main() -> Result<()> {
    println!(
        "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
        "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type", "Mount",
    );

    let mut partitions = disk::partitions_physical();
    while let Some(part) = await!(partitions.next()) {
        let part = part?;
        let usage = await!(disk::usage(part.mount_point().to_path_buf()))?;

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
