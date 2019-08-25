use heim_common::prelude::*;
use heim_disk as disk;

#[heim_derive::test]
async fn smoke_partitions() {
    let mut partitions = disk::partitions();
    while let Some(part) = partitions.next().await {
        let part = part.unwrap();

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
}

#[heim_derive::test]
async fn smoke_partitions_physical() {
    let mut partitions = disk::partitions_physical();
    while let Some(part) = partitions.next().await {
        let part = part.unwrap();

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
}

#[heim_derive::test]
async fn smoke_usage() {
    let usage = disk::usage("/").await;

    let usage = usage.unwrap();

    let _ = usage.total();
    let _ = usage.used();
    let _ = usage.free();
    let _ = usage.ratio();

    #[cfg(unix)]
    {
        use heim_disk::os::unix::UsageExt;

        let _ = usage.flags();
    }
}

#[heim_derive::test]
async fn smoke_io_counters() {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let _ = Command::new("diskperf").arg("-y").status();
    }

    let mut counters = disk::io_counters();
    while let Some(count) = counters.next().await {
        let count = count.unwrap();

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}

#[heim_derive::test]
async fn smoke_io_counters_physical() {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let _ = Command::new("diskperf").arg("-y").status();
    }

    let mut counters = disk::io_counters_physical();
    while let Some(count) = counters.next().await {
        let count = count.unwrap();

        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}
