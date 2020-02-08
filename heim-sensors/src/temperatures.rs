use std::fmt;

use heim_common::prelude::*;
use heim_common::units::ThermodynamicTemperature;

use crate::sys;

/// Hardware temperature sensor.
pub struct TemperatureSensor {
    pub(crate) unit: String,
    pub(crate) label: Option<String>,
    pub(crate) current: ThermodynamicTemperature,
    pub(crate) high: Option<ThermodynamicTemperature>,
    pub(crate) critical: Option<ThermodynamicTemperature>,
}

impl TemperatureSensor {
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns sensor label.
    #[allow(clippy::option_as_ref_deref)] // >= 1.40.0
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Returns current temperature reported by sensor.
    pub fn current(&self) -> ThermodynamicTemperature {
        self.current
    }

    /// Returns high trip point for sensor if available.
    pub fn high(&self) -> Option<ThermodynamicTemperature> {
        self.high
    }

    /// Returns critical trip point for sensor if available.
    pub fn critical(&self) -> Option<ThermodynamicTemperature> {
        self.critical
    }
}

impl fmt::Debug for TemperatureSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TemperatureSensor")
            .field("unit", &self.unit())
            .field("label", &self.label())
            .field("current", &self.current())
            .field("high", &self.high())
            .field("critical", &self.critical())
            .finish()
    }
}

/// Returns stream which yields [temperature sensors].
///
/// ## Compatibility
///
/// At the moment, this function works only with Linux.
/// For other platforms it returns an empty stream.
///
/// [temperature sensors]: ./struct.TemperatureSensor.html
pub fn temperatures() -> impl Stream<Item = Result<TemperatureSensor>> {
    sys::temperatures()
}
