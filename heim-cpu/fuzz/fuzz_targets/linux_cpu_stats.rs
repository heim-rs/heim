#![no_main]
#![cfg(target_os = "linux")]
use libfuzzer_sys::fuzz_target;

use std::str::FromStr;
use heim_cpu::sys::CpuStats;

fuzz_target!(|data: String| {
    let _ = CpuStats::from_str(&data);
});
