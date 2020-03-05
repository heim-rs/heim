use std::convert::TryFrom;
use std::io;

use winapi::shared::minwindef::DWORD;
use winapi::um::winbase;

use crate::os::windows::Priority;

impl TryFrom<DWORD> for Priority {
    type Error = io::Error;

    fn try_from(value: DWORD) -> io::Result<Priority> {
        match value {
            winbase::ABOVE_NORMAL_PRIORITY_CLASS => Ok(Priority::AboveNormal),
            winbase::BELOW_NORMAL_PRIORITY_CLASS => Ok(Priority::BelowNormal),
            winbase::HIGH_PRIORITY_CLASS => Ok(Priority::High),
            winbase::IDLE_PRIORITY_CLASS => Ok(Priority::Idle),
            winbase::NORMAL_PRIORITY_CLASS => Ok(Priority::Normal),
            winbase::REALTIME_PRIORITY_CLASS => Ok(Priority::RealTime),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown priority value {:x}", other),
            )),
        }
    }
}

impl From<Priority> for DWORD {
    fn from(value: Priority) -> DWORD {
        match value {
            Priority::AboveNormal => winbase::ABOVE_NORMAL_PRIORITY_CLASS,
            Priority::BelowNormal => winbase::BELOW_NORMAL_PRIORITY_CLASS,
            Priority::High => winbase::HIGH_PRIORITY_CLASS,
            Priority::Idle => winbase::IDLE_PRIORITY_CLASS,
            Priority::Normal => winbase::NORMAL_PRIORITY_CLASS,
            Priority::RealTime => winbase::REALTIME_PRIORITY_CLASS,
        }
    }
}
