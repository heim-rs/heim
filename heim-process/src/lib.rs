//! System processes information.
//!
//! This module is enabled with the `**process**` feature flag (enabled by default).

mod errors;
mod sys;

mod pids;

pub use self::pids::*;

pub use self::errors::ProcessError;

#[cfg(not(target_os = "windows"))]
pub type Pid = libc::pid_t;

// TODO: Is it a correct type for pid?
#[cfg(target_os = "windows")]
pub type Pid = winapi::shared::minwindef::DWORD;
