use heim_common::Pid;
use std::fmt;
use crate::ProcessResult;
#[derive(Default)]
pub struct IoCounters;


pub async fn io(_pid: Pid) -> ProcessResult<IoCounters> {
    ProcessResult::Ok(IoCounters)
}

impl fmt::Debug for IoCounters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IoCounters").finish()
    }
}