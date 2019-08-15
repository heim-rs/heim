use heim_common::units::Information;

/// Linux-specific extension to process [Memory] information.
///
/// [Memory]: ../../struct.Memory.html
#[heim_derive::os_ext_for(crate::Memory, cfg(target_os = "linux"))]
pub trait MemoryExt {
    /// Returns the amount of memory that could be potentially shared with other processes.
    fn shared(&self) -> Information;

    /// Returns TRS (*text resident set*) - the amount of memory devoted to executable code.
    fn text(&self) -> Information;

    /// Returns DRS (*data resident set*) - the amount of physical memory
    /// devoted to other than executable code.
    fn data(&self) -> Information;
}
