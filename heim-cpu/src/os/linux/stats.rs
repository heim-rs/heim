/// Linux-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
pub trait CpuStatsExt {
    /// Returns number of software interrupts since boot.
    fn soft_interrupts(&self) -> u64;
}

#[cfg(target_os = "linux")]
impl CpuStatsExt for crate::CpuStats {
    fn soft_interrupts(&self) -> u64 {
        self.as_ref().soft_interrupts()
    }
}
