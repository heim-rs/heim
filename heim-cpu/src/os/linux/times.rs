use crate::units::Time;

/// Linux-specific extension for [CpuTime].
///
/// [CpuTime]: crate::CpuTime
#[heim_derive::os_ext_for(crate::CpuTime, cfg(target_os = "linux"))]
pub trait CpuTimeExt {
    /// Returns time spent by niced (prioritized) processes executing in user mode,
    /// this also includes [guest_nice] time.
    ///
    /// [guest_nice]: #tymethod.guest_nice
    fn nice(&self) -> Time;

    /// Returns time spent waiting for I/O to complete.
    fn io_wait(&self) -> Time;

    /// Returns time spent for servicing hardware interrupts.
    fn irq(&self) -> Time;

    /// Returns time spent for servicing software interrupts.
    fn soft_irq(&self) -> Time;

    /// Returns time spent by other operating systems running in a virtualized environment.
    ///
    /// ## Compatibility
    ///
    /// Available for Linux 2.6.11+, older versions always returns `None`.
    fn steal(&self) -> Option<Time>;

    /// Returns time spent running a virtual CPU for guest operating systems
    /// under the control of the Linux kernel.
    ///
    /// ## Compatibility
    ///
    /// Available for Linux 2.6.24+, older versions always returns `None`.
    fn guest(&self) -> Option<Time>;

    /// Returns time spent running a niced guest
    /// (virtual CPU for guest operating systems under the control of the Linux kernel)
    ///
    /// ## Compatibility
    ///
    /// Available for Linux 3.2.0+, older versions always returns `None`.
    fn guest_nice(&self) -> Option<Time>;
}
