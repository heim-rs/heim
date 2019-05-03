/// Time measurement unit.
///
/// Base unit is `second`.
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Time(f64);

impl Time {
    #[doc(hidden)]
    pub fn from_nanoseconds(value: f64) -> Self {
        Self::new(value * 1_000_000_000.0)
    }

    #[doc(hidden)]
    pub fn from_milliseconds(value: f64) -> Self {
        Self::new(value * 1_000.0)
    }

    #[doc(hidden)]
    pub fn from_microseconds(value: f64) -> Self {
        Self::new(value * 1_000_000.0)
    }
}

/// Information measurement unit.
///
/// Base unit is `byte`.
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Information(u64);

impl Information {
    #[doc(hidden)]
    pub fn from_kilobytes(value: u64) -> Self {
        Self::new(value * 1_024)
    }
}

/// Ratio measurement unit.
///
/// It is dimensionless and represents the value in the `[0.0 … 1.0]` range
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Ratio(f32);
