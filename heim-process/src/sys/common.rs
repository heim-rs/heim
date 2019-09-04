use ordered_float::NotNan;

use crate::Pid;
use heim_common::units::{time, Time};

/// Process unique ID.
///
/// Processes can't be compared just by their PIDs,
/// as the PIDs can be re-used, so the minimal information amount
/// needed to unique identify a process is a (pid, create_time)
/// tuple.
///
/// In addition, since `create_time` is basically the `f64` type
/// internally, it is wrapped into a `NotNan` type
/// in order to provide `Hash`, `PartialEq` and `Eq` traits.
///
/// This struct is shared across multiple OS-specific implementations.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct UniqueId {
    pid: Pid,
    create_time: NotNan<f64>,
}

impl UniqueId {
    /// Create new `UniqueId` based on process' `pid` and `create_time`.
    ///
    /// ## Panics
    ///
    /// Will panic if `create_time` internally is `NaN`,
    /// which should be considered as a bug, since it should
    /// be impossible to get such a time value for process.
    pub fn new(pid: Pid, create_time: Time) -> UniqueId {
        let seconds = create_time.get::<time::second>();
        let time = NotNan::new(seconds).expect("Process create time can't be NaN");

        UniqueId {
            pid,
            create_time: time,
        }
    }

    /// Get back the process creation time.
    ///
    /// Mostly used to reduce `Process` struct size
    /// and re-use already loaded values
    pub fn create_time(&self) -> Time {
        Time::new::<time::second>(*self.create_time)
    }
}
