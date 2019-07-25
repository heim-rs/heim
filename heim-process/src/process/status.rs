/// Process status.
///
/// Returned by [Process::status] method.
///
/// [Process::status]: ./struct.Process.html#method.status
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Status {
    /// Running
    Running,

    /// Sleeping in an interruptible wait
    Sleeping,

    /// Waiting in uninterruptible disk sleep
    Waiting,

    /// Zombie
    Zombie,

    /// Stopped (on a signal)
    ///
    /// Or before Linux 2.6.33, trace stopped
    Stopped,

    /// Tracing stop (Linux 2.6.33 onward)
    Tracing,

    /// Dead
    Dead,

    /// Wakekill (Linux 2.6.33 to 3.13 only)
    Wakekill,

    /// Waking (Linux 2.6.33 to 3.13 only)
    Waking,

    /// Parked (P) (Linux 3.9 to 3.13 only)
    Parked,

    /// Idle
    ///
    /// ## Compatibility
    ///
    /// Applicable for Linux and macOS only.
    Idle,
}
