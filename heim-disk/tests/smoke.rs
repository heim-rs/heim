use heim_disk as disk;
use heim_runtime::{self as runtime, SyncRuntime};

#[test]
fn smoke_partitions() {
    let mut rt = runtime::new().unwrap();
    let partitions = rt.block_collect(disk::partitions());

    for part in partitions.flatten() {
        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();
        let _ = part.options();
    }
}

#[test]
fn smoke_partitions_physical() {
    let mut rt = runtime::new().unwrap();
    let partitions = rt.block_collect(disk::partitions_physical());

    for part in partitions.flatten() {
        let _ = part.device();
        let _ = part.mount_point();
        let _ = part.file_system();
        let _ = part.options();
    }
}

#[test]
fn smoke_usage() {
    let mut rt = runtime::new().unwrap();
    let usage = rt.block_run(disk::usage("/"));

    assert!(usage.is_ok());

    let usage = usage.unwrap();

    let _ = usage.total();
    let _ = usage.used();
    let _ = usage.free();
    let _ = usage.ratio();
}

#[test]
fn smoke_io_counters() {
    let mut rt = runtime::new().unwrap();
    let counters = rt.block_collect(disk::io_counters());

    for count in counters.flatten() {
        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}

#[test]
fn smoke_io_counters_physical() {
    let mut rt = runtime::new().unwrap();
    let counters = rt.block_collect(disk::io_counters_physical());

    for count in counters.flatten() {
        let _ = count.device_name();
        let _ = count.read_count();
        let _ = count.write_count();
        let _ = count.read_bytes();
        let _ = count.write_bytes();
    }
}
