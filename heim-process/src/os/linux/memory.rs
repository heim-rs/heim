use heim_common::units::Information;

/// Linux-specific extension to process [Memory] information.
///
/// [Memory]: ../../struct.Memory.html
pub trait MemoryExt {
    /// Returns the amount of memory that could be potentially shared with other processes.
    fn shared(&self) -> Information;

    /// Returns TRS (*text resident set*) - the amount of memory devoted to executable code.
    fn text(&self) -> Information;

    /// Returns DRS (*data resident set*) - the amount of physical memory
    /// devoted to other than executable code.
    fn data(&self) -> Information;
}

#[cfg(target_os = "linux")]
impl MemoryExt for crate::Memory {
    fn shared(&self) -> Information {
        self.as_ref().shared()
    }

    fn text(&self) -> Information {
        self.as_ref().text()
    }

    fn data(&self) -> Information {
        self.as_ref().data()
    }
}
