//! System processes information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

mod sys;

mod pids;

pub use self::pids::*;

pub type Pid = libc::pid_t;
