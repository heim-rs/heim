/// Linux-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
#[heim_derive::os_ext_for(crate::CpuStats, cfg(target_os = "linux"))]
pub trait CpuStatsExt {
    /// Returns number of software interrupts since boot.
    fn soft_interrupts(&self) -> u64;
}
