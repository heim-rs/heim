/// Windows-specific extension to process [Memory] information.
///
/// Information here is provided by the [PROCESS_MEMORY_COUNTERS_EX] struct,
/// see its documentation for details.
///
/// [Memory]: ../../struct.Memory.html
/// [PROCESS_MEMORY_COUNTERS_EX]: https://docs.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex
#[heim_derive::os_ext_for(crate::Memory, cfg(target_os = "windows"))]
pub trait MemoryExt {
    /// Returns the number of page faults.
    fn page_faults(&self) -> u32;

    /// Returns the the peak working set size.
    fn peak_working_set_size(&self) -> usize;

    /// Returns the current working set size.
    fn working_set_size(&self) -> usize;

    /// Returns the peak paged pool usage.
    fn quota_peak_paged_pool_usage(&self) -> usize;

    /// Returns the current paged pool usage.
    fn quota_paged_pool_usage(&self) -> usize;

    /// Returns the peak nonpaged pool usage.
    fn quota_peak_non_paged_pool_usage(&self) -> usize;

    /// Returns the current nonpaged pool usage.
    fn quota_non_paged_pool_usage(&self) -> usize;

    /// Returns the *Commit Charge* value in bytes.
    ///
    /// Commit Charge is the total amount of memory
    /// that the memory manager has committed for a running process.
    fn pagefile_usage(&self) -> usize;

    /// Returns the peak value in bytes of the *Commit Charge*
    /// during the lifetime of this process.
    fn peak_pagefile_usage(&self) -> usize;

    /// Same as [`pagefile_usage`](#tymethod.pagefile_usage).
    ///
    /// The *Commit Charge* value in bytes for this process.
    /// Commit Charge is the total amount of memory
    /// that the memory manager has committed for a running process.
    fn private_usage(&self) -> usize;
}
