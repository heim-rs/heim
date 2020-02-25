use heim_common::{
    sys::{macos::sysctl, IntoTime},
    units::Time,
    Result,
};

pub async fn boot_time() -> Result<Time> {
    let value: libc::timeval = sysctl::sysctl(&mut [libc::CTL_KERN, libc::KERN_BOOTTIME])?;

    Ok(value.into_time())
}
