//! Disks information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

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
