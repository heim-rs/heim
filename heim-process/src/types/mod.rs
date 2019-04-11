mod env;

pub use self::env::*;

/// Process identifier
pub type Pid = libc::pid_t;

/// Process state.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    TracingStop,
    Zombie,
    Dead,
    WakeKill,
    Waking,
    #[cfg(any(doc, target_os = "linux"))]
    Parked,
    #[cfg(any(doc, target_os = "linux", target_os = "freebsd", target_os = "dragonfly", target_os = "macos"))]
    Idle,
    #[cfg(any(doc, target_os = "freebsd", target_os = "dragonfly"))]
    Locked,
    #[cfg(any(doc, target_os = "freebsd", target_os = "dragonfly"))]
    Waiting,
    #[cfg(any(doc, target_os = "freebsd", target_os = "dragonfly"))]
    Suspended,
    #[doc(hidden)]
    __Nonexhaustive,
}

