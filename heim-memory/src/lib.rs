//! Memory information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#![allow(stable_features)]
#![feature(futures_api)]

mod memory;
pub mod os;
mod swap;
mod sys;

pub use self::memory::*;
pub use self::swap::*;

/// Re-exported measurement units used in this crate.
pub mod units {
    pub use heim_common::units::Information;
}
