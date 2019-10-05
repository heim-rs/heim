use heim_common::prelude::Result2;
use heim_common::sys::macos::sysctl;

pub async fn logical_count() -> Result2<u64> {
    let value: i32 = unsafe { sysctl::sysctlbyname(b"hw.logicalcpu\0")? };

    Ok(value as u64)
}

pub async fn physical_count() -> Result2<Option<u64>> {
    let value: i32 = unsafe { sysctl::sysctlbyname(b"hw.physicalcpu\0")? };

    Ok(Some(value as u64))
}
