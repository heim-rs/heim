use heim_common::{prelude::*, sys::macos::sysctl, sys::IntoTime, units::Time};

pub async fn boot_time() -> Result<Time> {
    let mut name = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let time: libc::timeval =
        sysctl::sysctl(&mut name).map_err(|e| Error::from(e).with_sysctl(&name))?;

    Ok(time.into_time())
}
