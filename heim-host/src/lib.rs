//! Information about host system.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#[cfg(target_os = "windows")]
#[macro_use]
extern crate winapi;

pub use platforms::target::Arch;

mod sys;

pub mod os;
mod platform;
mod uptime;
mod users;

pub use self::platform::*;
pub use self::uptime::*;
pub use self::users::*;

#[cfg(any(doc, not(target_os = "windows")))]
type Pid = libc::pid_t;

/// Re-exported measurement units used in this crate.
pub mod units {
    pub use heim_common::units::si::f64::Time;
}
