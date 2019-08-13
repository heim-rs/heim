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
