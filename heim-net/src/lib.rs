//! Network information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

#![deny(unused)]
#![warn(missing_docs)]
#![deny(unstable_features)]
#![deny(bare_trait_objects)]

mod sys;

mod counters;
mod nic;

pub use self::counters::*;
pub use self::nic::*;
