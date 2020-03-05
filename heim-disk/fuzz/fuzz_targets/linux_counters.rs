#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str::FromStr;
use heim_disk::sys::IoCounters;

fuzz_target!(|data: String| {
    let _ = IoCounters::from_str(&data);
});
