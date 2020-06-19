use heim_common::prelude::Result;
use heim_common::sys::macos::sysctl;

pub async fn logical_count() -> Result<u64> {
    // sysctl value has i32 type
    unsafe { sysctl::sysctlbyname::<i32>(b"hw.logicalcpu\0").map(|v| v as u64) }
}

pub async fn physical_count() -> Result<Option<u64>> {
    unsafe { sysctl::sysctlbyname::<i32>(b"hw.physicalcpu\0").map(|v| Some(v as u64)) }
}
