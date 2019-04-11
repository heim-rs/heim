//! CPU information.
//!
//! This crate is a part of [heim](https://crates.io/crates/heim) project,
//! consider using it instead.

pub mod os;
mod sys;

mod freq;
mod stats;
mod times;

pub use self::freq::*;
pub use self::stats::*;
pub use self::times::*;

/// Re-exported measurement units used by this crate.
pub mod units {
    pub use heim_common::units::si::f64::Time;
    pub use heim_common::units::si::u64::Frequency;
    pub use heim_common::units::si::Quantity;

    pub use heim_common::units::si::frequency::*;
    pub use heim_common::units::si::time::*;
}
