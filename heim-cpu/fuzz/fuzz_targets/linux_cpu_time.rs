#![no_main]
#![cfg(target_os = "linux")]
use libfuzzer_sys::fuzz_target;

use std::str::FromStr;
use heim_cpu::sys::CpuTime;

fuzz_target!(|data: String| {
    let _ = CpuTime::from_str(&data);
});
