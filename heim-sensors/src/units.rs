/// Temperature measurement unit.
///
/// Base unit is Celsius degree.
#[derive(heim_derive::Unit, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Temperature(f64);

impl Temperature {
    #[doc(hidden)]
    pub fn from_millidegrees(value: f64) -> Temperature {
        Self(value / 1_000.0)
    }
}
