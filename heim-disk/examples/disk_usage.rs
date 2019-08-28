//! Command similar to `df -BM`

use std::ffi::OsStr;

use heim_common::prelude::*;
use heim_common::units::information;
use heim_disk as disk;

#[heim_derive::main]
async fn main() -> Result<()> {
    println!(
        "{:<17} {:<10} {:<10} {:<10} {:<10} Mount",
        "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type"
    );

    let mut partitions = disk::partitions_physical();
    while let Some(part) = partitions.next().await {
        let part = part?;
        let usage = disk::usage(part.mount_point().to_path_buf()).await?;

        println!(
            "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
            part.device()
                .unwrap_or_else(|| OsStr::new("N/A"))
                .to_string_lossy(),
            usage.total().get::<information::megabyte>(),
            usage.used().get::<information::megabyte>(),
            usage.free().get::<information::megabyte>(),
            part.file_system().as_str(),
            part.mount_point().to_string_lossy(),
        );
    }

    Ok(())
}
