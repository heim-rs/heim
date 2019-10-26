/// macOS-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
pub trait CpuStatsExt {
    /// Returns number of software interrupts since boot.
    fn soft_interrupts(&self) -> u64;

    /// Returns number of syscalls since boot.
    fn syscalls(&self) -> u64;
}

#[cfg(target_os = "macos")]
impl CpuStatsExt for crate::CpuStats {
    fn soft_interrupts(&self) -> u64 {
        self.as_ref().soft_interrupts()
    }

    fn syscalls(&self) -> u64 {
        self.as_ref().syscalls()
    }
}
