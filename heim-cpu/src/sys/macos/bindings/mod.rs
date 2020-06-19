use heim_common::prelude::Result;
use heim_common::sys::macos::sysctl;

mod cpu;
mod processor;
mod vm;

pub use self::cpu::{cpu_load_info, host_cpu_load_info};
pub use self::processor::{processor_cpu_load_info, processor_load_info};
pub use self::vm::{vm_meter, vmmeter};

// Returns hertz
pub fn cpu_frequency() -> Result<u64> {
    // sysctl frequency value has `i64` type
    unsafe { sysctl::sysctlbyname::<i64>(b"hw.cpufrequency\0").map(|v| v as u64) }
}

// Returns hertz
pub fn cpu_frequency_max() -> Result<u64> {
    // sysctl frequency value has `i64` type
    unsafe { sysctl::sysctlbyname::<i64>(b"hw.cpufrequency_max\0").map(|v| v as u64) }
}

// Returns hertz
pub fn cpu_frequency_min() -> Result<u64> {
    // sysctl frequency value has `i64` type
    unsafe { sysctl::sysctlbyname::<i64>(b"hw.cpufrequency_min\0").map(|v| v as u64) }
}
