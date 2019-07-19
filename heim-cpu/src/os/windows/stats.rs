/// Windows-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
#[heim_derive::os_ext_for(crate::CpuStats, cfg(target_os = "windows"))]
pub trait CpuStatsExt {
    /// Returns number of [Deferred Procedure Calls] since boot.
    ///
    /// [Deferred Procedure Calls]: https://en.wikipedia.org/wiki/Deferred_Procedure_Call
    fn dpc(&self) -> u64;

    /// Returns number of syscalls since boot.
    fn syscalls(&self) -> u64;
}
