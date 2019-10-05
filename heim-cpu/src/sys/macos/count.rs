use heim_common::prelude::{Error2 as Error, Result2};
use heim_common::sys::macos::sysctl;

pub async fn logical_count() -> Result2<u64> {
    let value: i32 = unsafe {
        sysctl::sysctlbyname(b"hw.logicalcpu\0")
            .map_err(|e| Error::from(e).with_named_syscall("hw.logicalcpu"))
    }?;

    Ok(value as u64)
}

pub async fn physical_count() -> Result2<Option<u64>> {
    let value: i32 = unsafe {
        sysctl::sysctlbyname(b"hw.physicalcpu\0")
            .map_err(|e| Error::from(e).with_named_syscall("hw.physicalcpu"))
    }?;

    Ok(Some(value as u64))
}
