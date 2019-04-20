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
    // The obvious choice is too use `usize` type as a memory size representation.
    // Yet, the problem is `Information` base unit is "bit", not a "byte",
    // therefore all the time we are having the multiplication by 8,
    // and this breaks `x86` systems, because even small x86 VMs
    // have so much memory bits, that it overflows their `usize`
    // (which is `u32` in that case).
    //
    // Therefore, we are stuck with a `u64` for all systems.
    pub use heim_common::units::iec::u64::Information;
}
