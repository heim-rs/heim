use std::error::Error;

use futures::StreamExt;

use heim_disk as disk;

#[heim_derive::test]
async fn smoke_partitions() -> Result<(), Box<dyn Error>> {
    let partitions = disk::partitions().await?;
    futures::pin_mut!(partitions);
    while let Some(part) = partitions.next().await {
        let part = part?;

        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();

        #[cfg(target_os = "macos")]
        {
            use heim_disk::os::macos::PartitionExt;

            let _ = part.flags();
        }

        #[cfg(target_os = "windows")]
        {
            use heim_disk::os::windows::PartitionExt;

            let _ = part.flags();
            let _ = part.drive_type();
        }
    }

    Ok(())
}

#[heim_derive::test]
async fn smoke_partitions_physical() -> Result<(), Box<dyn Error>> {
    let partitions = disk::partitions_physical().await?;
    futures::pin_mut!(partitions);
    while let Some(part) = partitions.next().await {
        let part = part?;

        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();

        #[cfg(target_os = "macos")]
        {
            use heim_disk::os::macos::PartitionExt;

            let _ = part.flags();
        }

        #[cfg(target_os = "windows")]
        {
            use heim_disk::os::windows::PartitionExt;

            let _ = part.flags();
            let _ = part.drive_type();
        }
    }

    Ok(())
}

#[heim_derive::test]
async fn smoke_usage() -> Result<(), Box<dyn Error>> {
    let usage = disk::usage("/").await?;

    let _ = usage.total();
    let _ = usage.used();
    let _ = usage.free();
    let _ = usage.ratio();

    #[cfg(unix)]
    {
        use heim_disk::os::unix::UsageExt;

        let _ = usage.flags();
    }

    Ok(())
}

#[heim_derive::test]
async fn smoke_io_counters() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let _ = Command::new("diskperf").arg("-y").status();
    }

    let counters = disk::io_counters().await?;
    futures::pin_mut!(counters);
    while let Some(count) = counters.next().await {
        let count = count.unwrap();

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }

    Ok(())
}

#[heim_derive::test]
async fn smoke_io_counters_physical() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let _ = Command::new("diskperf").arg("-y").status();
    }

    let counters = disk::io_counters_physical().await?;
    futures::pin_mut!(counters);
    while let Some(count) = counters.next().await {
        let count = count?;

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }

    Ok(())
}
