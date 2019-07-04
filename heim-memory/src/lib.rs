//! Memory and swap information.
//!
//! This module is enabled with the `**memory**` feature flag (enabled by default).

#![allow(stable_features)]
#![feature(futures_api)]

mod memory;
pub mod os;
mod swap;
mod sys;

pub use self::memory::*;
pub use self::swap::*;

pub use heim_common::units::Information;
