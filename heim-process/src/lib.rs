//! System processes information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

mod sys;

mod pids;

pub use self::pids::*;

#[cfg(not(target_os = "windows"))]
pub type Pid = libc::pid_t;

// TODO: Is it a correct type for pid?
#[cfg(target_os = "windows")]
pub type Pid = winapi::shared::minwindef::DWORD;
