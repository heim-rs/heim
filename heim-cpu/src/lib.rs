//! CPU information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#![allow(stable_features)]
#![feature(futures_api)]

pub mod os;
mod sys;

mod freq;
mod stats;
mod times;
mod units;

pub use self::freq::*;
pub use self::stats::*;
pub use self::times::*;
pub use self::units::*;
