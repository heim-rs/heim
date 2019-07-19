/// macOS-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
#[heim_derive::os_ext_for(crate::CpuStats, cfg(target_os = "macos"))]
pub trait CpuStatsExt {
    /// Returns number of software interrupts since boot.
    fn soft_interrupts(&self) -> u64;

    /// Returns number of syscalls since boot.
    fn syscalls(&self) -> u64;
}
