use heim_common::units::Time;

#[derive(Debug)]
pub struct CpuTime {
    pub(crate) user: Time,
    pub(crate) kernel: Time,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.user
    }

    pub fn system(&self) -> Time {
        self.kernel
    }
}
