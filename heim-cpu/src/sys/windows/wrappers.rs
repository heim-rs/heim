#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::io;
use std::mem;
use std::ptr;

use ntapi::ntexapi::{
    NtQuerySystemInformation, SystemInterruptInformation, SystemPerformanceInformation,
    SystemProcessorPerformanceInformation, SYSTEM_INFORMATION_CLASS, SYSTEM_INTERRUPT_INFORMATION,
    SYSTEM_PERFORMANCE_INFORMATION, SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION,
};
use ntapi::ntpoapi::PROCESSOR_POWER_INFORMATION;
use winapi::shared::{minwindef, ntstatus};
use winapi::um::{powerbase, sysinfoapi, winnt};

use heim_common::prelude::{Error, Result};

pub trait SystemInformation: Sized {
    fn class() -> SYSTEM_INFORMATION_CLASS;
}

impl SystemInformation for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn class() -> SYSTEM_INFORMATION_CLASS {
        SystemProcessorPerformanceInformation
    }
}

impl SystemInformation for SYSTEM_PERFORMANCE_INFORMATION {
    fn class() -> SYSTEM_INFORMATION_CLASS {
        SystemPerformanceInformation
    }
}

impl SystemInformation for SYSTEM_INTERRUPT_INFORMATION {
    fn class() -> SYSTEM_INFORMATION_CLASS {
        SystemInterruptInformation
    }
}

// TODO: This one can be cached in the `lazy_static`
pub fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();

    unsafe {
        sysinfoapi::GetSystemInfo(info.as_mut_ptr());

        info.assume_init()
    }
}

// Safe wrapper around the `NtQuerySystemInformation`
pub fn query_system_information<T>() -> Result<Vec<T>>
where
    T: SystemInformation,
{
    let info = get_system_info();
    let proc_amount = info.dwNumberOfProcessors as usize;
    if proc_amount == 0 {
        let e = Error::from(io::Error::from(io::ErrorKind::NotFound))
            .with_message("GetSystemInfo returned zero CPUs");
        return Err(e);
    }

    let mut info = Vec::<T>::with_capacity(proc_amount);
    let buffer_length = proc_amount * mem::size_of::<T>();

    unsafe {
        let result = NtQuerySystemInformation(
            T::class(),
            info.as_mut_ptr() as *mut _,
            buffer_length as u32,
            ptr::null_mut(),
        );
        if result != ntstatus::STATUS_SUCCESS {
            return Err(Error::last_os_error().with_ffi("NtQuerySystemInformation"));
        }
        info.set_len(proc_amount);
    };

    debug_assert!(!info.is_empty());

    Ok(info)
}

pub fn get_processors() -> Result<Vec<PROCESSOR_POWER_INFORMATION>> {
    let info = get_system_info();
    if info.dwNumberOfProcessors == 0 {
        let e = Error::from(io::Error::from(io::ErrorKind::NotFound))
            .with_message("GetSystemInfo returned zero CPUs");
        return Err(e);
    }

    let proc_amount = info.dwNumberOfProcessors as usize;
    let mut processors = Vec::<PROCESSOR_POWER_INFORMATION>::with_capacity(proc_amount);
    let buffer_length = proc_amount * mem::size_of::<PROCESSOR_POWER_INFORMATION>();

    let result = unsafe {
        powerbase::CallNtPowerInformation(
            winnt::ProcessorInformation,
            ptr::null_mut(),
            0,
            processors.as_mut_ptr() as *mut _,
            buffer_length as minwindef::ULONG,
        )
    };

    if result == ntstatus::STATUS_SUCCESS {
        unsafe { processors.set_len(proc_amount) };

        Ok(processors)
    } else {
        Err(Error::last_os_error().with_ffi("CallNtPowerInformation"))
    }
}
