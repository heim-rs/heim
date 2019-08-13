use heim_common::units::Time;

use super::Stat;

#[derive(Debug)]
pub struct CpuTime {
    utime: Time,
    stime: Time,
    children_utime: Time,
    children_stime: Time,
}

impl CpuTime {
    pub fn user(&self) -> Time {
        self.utime
    }

    pub fn system(&self) -> Time {
        self.stime
    }
}

impl From<Stat> for CpuTime {
    fn from(stat: Stat) -> CpuTime {
        CpuTime {
            utime: stat.utime,
            stime: stat.stime,
            children_utime: stat.cutime,
            children_stime: stat.cstime,
        }
    }
}
