use heim_common::units::Time;

#[derive(Debug)]
pub struct CpuTime {
    utime: Time,
    stime: Time,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.utime
    }

    pub fn system(&self) -> Time {
        self.stime
    }
}

impl From<darwin_libproc::proc_taskinfo> for CpuTime {
    fn from(info: darwin_libproc::proc_taskinfo) -> CpuTime {
        CpuTime {
            utime: Time::from_nanoseconds(info.pti_total_user as f64),
            stime: Time::from_nanoseconds(info.pti_total_system as f64),
        }
    }
}
