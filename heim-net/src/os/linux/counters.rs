//! Linux-specific extensions.
//!
//! Available only for `cfg(target_os = "linux")`

/// Linux-specific extension for [IoCounters].
///
/// [IoCounters]: ../../struct.IoCounters.html
#[heim_derive::os_ext_for(crate::IoCounters, cfg(target_os = "linux"))]
pub trait IoCountersExt {
    /// Returns packets amount which were dropped while sending them.
    fn drop_sent(&self) -> u64;
}
