use std::io;
use std::ptr;

use winapi::shared::{minwindef, winerror};
use winapi::um::{sysinfoapi, winnt};

/// This struct contains information about logical processors
/// received from the `GetLogicalProcessorInformationEx` function.
///
/// Major problem is that each "Processor" struct in it
/// has the variable size (described by the `Size` field),
/// so we are storing data as a plain bytes vector
/// and unsafely iterating on it later.
#[derive(Debug)]
pub struct LogicalProcessors {
    buffer: Vec<u8>,
}

impl LogicalProcessors {
    // TODO: Ensure that allowing this lint is fine.
    // It is triggered by casting `*mut u8` to the logical processor info struct pointer,
    // which is `#[repr(C)]` and generally should be fine.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn get() -> io::Result<Self> {
        let mut buffer = vec![];
        let mut buffer_size = 0u32;

        let result = unsafe {
            sysinfoapi::GetLogicalProcessorInformationEx(
                winnt::RelationAll,
                ptr::null_mut(),
                &mut buffer_size,
            )
        };
        debug_assert_eq!(result, minwindef::FALSE);

        loop {
            // Allocating enough memory to fill the buffer now
            buffer.reserve(buffer_size as usize - buffer.capacity());

            let result = unsafe {
                sysinfoapi::GetLogicalProcessorInformationEx(
                    winnt::RelationAll,
                    buffer.as_mut_ptr() as *mut _,
                    &mut buffer_size,
                )
            };

            if result == minwindef::FALSE {
                let e = io::Error::last_os_error();
                match e.raw_os_error() {
                    // Slight chance that there is now more CPU cores
                    // and we need more memory?
                    Some(value) if value == winerror::ERROR_INSUFFICIENT_BUFFER as i32 => continue,
                    _ => return Err(e),
                }
            } else {
                unsafe {
                    buffer.set_len(buffer_size as usize);
                }
                break;
            }
        }

        Ok(Self { buffer })
    }

    pub fn iter(&self) -> LogicalProcessorsIter<'_> {
        LogicalProcessorsIter {
            data: &self.buffer,
            offset: 0,
        }
    }
}

#[derive(Debug)]
pub struct LogicalProcessorsIter<'p> {
    data: &'p [u8],
    offset: usize,
}

impl<'p> Iterator for LogicalProcessorsIter<'p> {
    type Item = &'p winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;

    #[allow(clippy::cast_ptr_alignment)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.data.len() {
            return None;
        }

        let core = unsafe {
            let ptr = self.data.as_ptr().add(self.offset)
                as winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;
            self.offset += (*ptr).Size as usize;
            &*ptr
        };
        Some(core)
    }
}
