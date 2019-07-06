//! Command similar to `df -BM`

#![feature(async_await)]

use std::ffi::OsStr;

use heim_common::prelude::*;
use heim_disk as disk;

const MEGABYTE: u64 = 1_024 * 1_024;

#[runtime::main]
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
            part.device().unwrap_or_else(|| OsStr::new("N/A")).to_string_lossy(),
            usage.total().get() / MEGABYTE,
            usage.used().get() / MEGABYTE,
            usage.free().get() / MEGABYTE,
            part.file_system().as_str(),
            part.mount_point().to_string_lossy(),
        );
    }

    Ok(())
}
