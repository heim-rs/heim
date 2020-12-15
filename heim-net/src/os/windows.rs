//! Windows-specific extensions.
//!
//! Available only for `cfg(target_os = "windows")`


/// Windows-specific extension for [Nic].
///
/// [Nic]: ../../struct.Nic.html
pub trait NicExt {
    /// Returns NIC GUID
    fn guid(&self) -> &str;
}

#[cfg(target_os = "windows")]
impl NicExt for crate::Nic {
    fn guid(&self) -> &str {
        self.as_ref().guid()
    }
}


/// Windows-specific extension for [IoCounters].
///
/// [IoCounters]: ../../struct.IoCounters.html
pub trait IoCountersExt {
    /// Returns packets amount which were dropped while sending them.
    fn drop_sent(&self) -> u64;
}

#[cfg(target_os = "windows")]
impl IoCountersExt for crate::IoCounters {
    fn drop_sent(&self) -> u64 {
        self.as_ref().drop_sent()
    }
}
