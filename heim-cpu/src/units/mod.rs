//! Re-exported measurement units used by this crate.

pub use heim_common::units::Time;

/// Frequency measurement unit.
///
/// Base unit is `hertz`.
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Frequency(u64);

#[doc(hidden)]
impl Frequency {
    pub fn from_megahertzs(value: u64) -> Self {
        Self::new(value * 1_000_000)
    }
}
