use std::mem;

use winapi::shared::minwindef;
use winapi::um::sysinfoapi;

use heim_common::prelude::*;
use heim_common::sys::IntoTime;
use heim_common::units::{time, Time};

pub async fn boot_time() -> Result<Time> {
    let mut filetime = mem::MaybeUninit::<minwindef::FILETIME>::uninit();

    // `time` value is now a time amount from the January 1, 1601
    let time = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime
        // function returns nothing and can't fail, apparently
        sysinfoapi::GetSystemTimeAsFileTime(filetime.as_mut_ptr());
        filetime.assume_init().into_time()
    };

    let elapsed_since_boot = Time::new::<time::millisecond>(unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64
        // function returns an ULONGLONG and can't fail, apparently
        sysinfoapi::GetTickCount64() as f64
    });

    // Seconds amount between the "Windows epoch" (January 1, 1601)
    // and the Unix epoch (January 1, 1970).
    // TODO: It would be nice to make it const,
    // as soon as `uom` will mark `Time::new` as a `const fn`
    let unix_epoch_delta = Time::new::<time::second>(11_644_473_600.0);

    Ok(time - elapsed_since_boot - unix_epoch_delta)
}

#[cfg(test)]
mod test {
    use crate::boot_time;
    use std::time::{SystemTime, Duration};

    #[test]
    fn check_boot_time() {
        let current_timestamp = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64();
        std::thread::sleep(Duration::from_secs(1));
        let boot_timestamp = smol::block_on(boot_time()).unwrap().value;
        assert!(boot_timestamp < current_timestamp, "Boot time is greater than current time. Boot timestamp: {}, current timestamp: {}", boot_timestamp, current_timestamp);
    }
}
