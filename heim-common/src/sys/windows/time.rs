use winapi::shared::{minwindef, ntdef};

use crate::units;

const HI_T: f64 = 429.496_729_6;
const LO_T: f64 = 1e-7;

/// Converting Windows structs into the time unit
pub trait IntoTime {
    /// Do the magic
    fn into_time(self) -> units::Time;
}

impl IntoTime for minwindef::FILETIME {
    #[inline]
    fn into_time(self) -> units::Time {
        let value = (HI_T * f64::from(self.dwHighDateTime))
            + (LO_T * f64::from(self.dwLowDateTime));

        units::Time::new(value)
    }
}

impl IntoTime for ntdef::LARGE_INTEGER {
    #[inline]
    fn into_time(self) -> units::Time {
        let s = unsafe { self.s() };
        let value = (HI_T * f64::from(s.HighPart))
            + (LO_T * f64::from(s.LowPart));

        units::Time::new(value)
    }
}
