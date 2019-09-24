//! Some external definitions that are missing in the `mach`, `libc` and `nix` crates.

#![allow(non_camel_case_types)]

use mach::kern_return::kern_return_t;
use mach::mach_types::{host_name_port_t, host_t};
use mach::message::mach_msg_type_number_t;
use mach::vm_types::integer_t;

use super::IntoTime;
use crate::units::{time, Time};

pub mod iokit;

/// https://developer.apple.com/documentation/kernel/host_flavor_t?language=objc
pub type host_flavor_t = integer_t;

/// https://developer.apple.com/documentation/kernel/host_info_t?language=objc
pub type host_info_t = *mut integer_t;
/// https://developer.apple.com/documentation/kernel/host_info64_t?language=objc
pub type host_info64_t = *mut integer_t;

extern "C" {
    pub fn mach_host_self() -> host_name_port_t;

    /// https://developer.apple.com/documentation/kernel/1502546-host_statistics?language=objc
    pub fn host_statistics(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info_out: host_info_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;

    /// https://developer.apple.com/documentation/kernel/1502863-host_statistics64?language=objc
    pub fn host_statistics64(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info_out: host_info64_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;
}

impl IntoTime for libc::timeval {
    fn into_time(self) -> Time {
        Time::new::<time::second>(self.tv_sec as f64)
            + Time::new::<time::microsecond>(f64::from(self.tv_usec))
    }
}
