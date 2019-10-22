use winapi::shared::{minwindef, ntdef};

use crate::sys::IntoTime;
use crate::units::{time, Time};

const HI_T: f64 = 429.496_729_6;
const LO_T: f64 = 1e-7;

impl IntoTime for minwindef::FILETIME {
    #[inline]
    fn into_time(self) -> Time {
        let low = LO_T * f64::from(self.dwLowDateTime);
        let value = HI_T.mul_add(f64::from(self.dwHighDateTime), low);

        Time::new::<time::second>(value)
    }
}

impl IntoTime for ntdef::LARGE_INTEGER {
    #[inline]
    fn into_time(self) -> Time {
        let s = unsafe { self.s() };
        let low = LO_T * f64::from(s.LowPart);
        let value = HI_T.mul_add(f64::from(s.HighPart), low);

        Time::new::<time::second>(value)
    }
}
