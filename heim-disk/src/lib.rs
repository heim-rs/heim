//! Disks information.
//!
//! This module is enabled with the `**disk**` feature flag (enabled by default).

#![allow(stable_features)]
#![feature(futures_api)]

pub mod os;
mod sys;

mod counters;
mod filesystem;
mod partitions;
mod usage;

pub use self::counters::*;
pub use self::filesystem::*;
pub use self::partitions::*;
pub use self::usage::*;

pub use heim_common::units::Information;
pub use heim_common::units::Ratio;
