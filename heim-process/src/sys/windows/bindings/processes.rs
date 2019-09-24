/// ## References
///
/// https://www.geoffchappell.com/studies/windows/km/ntoskrnl/api/ex/sysinfo/process.htm?ts=0,1346
use std::slice;

use heim_common::prelude::{Error, Result};
use heim_common::sys::windows as ntdll;

use ntapi::ntexapi;
use winapi::shared::{minwindef, ntdef, ntstatus};

#[derive(Debug)]
pub struct NtProcesses {
    data: Vec<u8>,
}

impl NtProcesses {
    pub fn load() -> Result<NtProcesses> {
        let mut data = Vec::with_capacity(0x4000);
        let mut needed_size = 0;

        loop {
            let result = unsafe {
                ntdll::NtQuerySystemInformation(
                    ntdll::SystemProcessInformation,
                    data.as_mut_ptr() as ntdef::PVOID,
                    data.capacity() as minwindef::ULONG,
                    &mut needed_size,
                )?
            };

            match result {
                ntstatus::STATUS_SUCCESS => {
                    unsafe { data.set_len(needed_size as usize) };
                    debug_assert!(data.capacity() >= data.len());
                    break;
                }
                ntstatus::STATUS_BUFFER_TOO_SMALL | ntstatus::STATUS_INFO_LENGTH_MISMATCH => {
                    data.reserve(needed_size as usize);
                    continue;
                }
                _ => return Err(Error::last_os_error()),
            }
        }

        Ok(NtProcesses { data })
    }

    pub fn iter(&self) -> NtProcessesIter {
        NtProcessesIter { data: &self.data }
    }
}

#[derive(Debug)]
pub struct NtProcessesIter<'p> {
    data: &'p [u8],
}

impl<'p> Iterator for NtProcessesIter<'p> {
    type Item = NtProcess<'p>;

    #[allow(clippy::cast_ptr_alignment)]
    fn next(&mut self) -> Option<Self::Item> {
        let process = self.data.as_ptr() as *const ntexapi::SYSTEM_PROCESS_INFORMATION;
        debug_assert!(unsafe { (*process).NextEntryOffset } as usize <= self.data.len());

        self.data = unsafe {
            let offset = (*process).NextEntryOffset as usize;
            #[cfg(target_pointer_width = "32")]
            {
                debug_assert!(offset == 0 || offset >= 0xb8);
            }

            #[cfg(target_pointer_width = "64")]
            {
                debug_assert!(offset == 0 || offset >= 0x100);
            }
            self.data.get_unchecked(offset..)
        };

        let threads = unsafe {
            slice::from_raw_parts(
                (*process).Threads.as_ptr(),
                (*process).NumberOfThreads as usize,
            )
        };

        Some(NtProcess {
            process: unsafe { &*process },
            threads,
        })
    }
}

pub struct NtProcess<'p> {
    pub process: &'p ntexapi::SYSTEM_PROCESS_INFORMATION,
    pub threads: &'p [ntexapi::SYSTEM_THREAD_INFORMATION],
}
