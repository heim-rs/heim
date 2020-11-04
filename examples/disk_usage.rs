//! Unix `du -h` command implementation.

use std::error::Error;
use std::ffi::OsStr;

use futures::StreamExt;
use heim::units::information;

fn main() -> Result<(), Box<dyn Error>> {
    smol::block_on(async {
        println!(
            "{:<17} {:<10} {:<10} {:<10} {:<10} Mount",
            "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type"
        );

        let partitions = heim::disk::partitions_physical().await?;
        futures::pin_mut!(partitions);

        while let Some(part) = partitions.next().await {
            let part = part?;
            let usage = heim::disk::usage(part.mount_point().to_path_buf()).await?;

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
    })
}
