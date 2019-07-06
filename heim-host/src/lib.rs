//! Host system information.
//!
//! This module is enabled with the `host` feature flag (enabled by default).

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

pub use heim_common::units::Time;
