/// macOS-specific extension to process [Memory] information.
///
/// [Memory]: ../../struct.Memory.html
#[heim_derive::os_ext_for(crate::Memory, cfg(target_os = "macos"))]
pub trait MemoryExt {
    /// Returns the amount of page faults.
    fn faults(&self) -> u64;

    /// Returns the amount of actual pageins.
    fn pageins(&self) -> u64;
}
