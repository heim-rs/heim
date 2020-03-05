#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str::FromStr;
use heim_disk::sys::Partition;

fuzz_target!(|data: String| {
    let _ = Partition::from_str(&data);
});
