use heim_common::units::Time;

/// Linux-specific extension for [CpuTime].
///
/// [CpuTime]: ../../struct.CpuTime.html
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
    fn steal(&self) -> Time;

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

impl CpuTimeExt for crate::CpuTime {
    fn nice(&self) -> Time {
        self.as_ref().nice()
    }

    fn io_wait(&self) -> Time {
        self.as_ref().io_wait()
    }

    fn irq(&self) -> Time {
        self.as_ref().irq()
    }

    fn soft_irq(&self) -> Time {
        self.as_ref().soft_irq()
    }

    fn steal(&self) -> Time {
        self.as_ref().steal()
    }

    fn guest(&self) -> Option<Time> {
        self.as_ref().guest()
    }

    fn guest_nice(&self) -> Option<Time> {
        self.as_ref().guest_nice()
    }
}
