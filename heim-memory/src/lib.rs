//! Memory information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#![deny(unused)]
#![warn(missing_docs)]
#![deny(unstable_features)]
#![deny(bare_trait_objects)]

mod memory;
pub mod os;
mod swap;
mod sys;

pub use self::memory::*;
pub use self::swap::*;

/// Re-exported measurement units used in this crate.
pub mod units {
    pub use heim_common::units::iec::information::*;
    pub use heim_common::units::iec::usize::Information;
}
