/// Windows-specific extension for [CpuStats].
///
/// [CpuStats]: ../../struct.CpuStats.html
pub trait CpuStatsExt {
    /// Returns number of [Deferred Procedure Calls] since boot.
    ///
    /// [Deferred Procedure Calls]: https://en.wikipedia.org/wiki/Deferred_Procedure_Call
    fn dpc(&self) -> u64;

    /// Returns number of syscalls since boot.
    fn syscalls(&self) -> u64;
}

#[cfg(target_os = "windows")]
impl CpuStatsExt for crate::CpuStats {
    fn dpc(&self) -> u64 {
        self.as_ref().dpc()
    }

    fn syscalls(&self) -> u64 {
        self.as_ref().syscalls()
    }
}
