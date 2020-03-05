//! Process priority class values.
//!
//! Note that `PROCESS_MODE_BACKGROUND_BEGIN` and `PROCESS_MODE_BACKGROUND_END`
//! are not implemented in here, as they are not priority classes at all,
//! can't be returned by `GetPriorityClass` and also breaks the `Eq` implementation.

/// Process priority class.
///
/// See [`GetPriorityClass`] function for more information.
///
/// [`GetPriorityClass`]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /// Process whose threads run only when the system is idle.
    ///
    /// The threads of the process are preempted by the threads
    /// of any process running in a higher priority class.
    Idle,

    /// Process that has priority above `Idle` but below `Normal`.
    BelowNormal,

    /// Process with no special scheduling needs.
    Normal,

    /// Process that has priority above `Normal` but below `High`.
    AboveNormal,

    /// Process that performs time-critical tasks that must be executed immediately.
    ///
    /// The threads of the process preempt the threads of normal or idle priority class processes.
    High,

    /// Process that has the highest possible priority.
    ///
    /// The threads of the process preempt the threads of all other processes,
    /// including operating system processes performing important tasks.
    RealTime,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::Priority;

    #[test]
    fn test_ord() {
        assert!(Priority::Idle < Priority::BelowNormal);
        assert!(Priority::Idle < Priority::Normal);
        assert!(Priority::Idle < Priority::AboveNormal);
        assert!(Priority::Idle < Priority::High);
        assert!(Priority::Idle < Priority::RealTime);
    }
}
