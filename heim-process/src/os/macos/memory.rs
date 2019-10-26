/// macOS-specific extension to process [Memory] information.
///
/// [Memory]: ../../struct.Memory.html
pub trait MemoryExt {
    /// Returns the amount of page faults.
    fn faults(&self) -> u64;

    /// Returns the amount of actual pageins.
    fn pageins(&self) -> u64;
}

impl MemoryExt for crate::Memory {
    fn faults(&self) -> u64 {
        self.as_ref().faults()
    }

    fn pageins(&self) -> u64 {
        self.as_ref().pageins()
    }
}
