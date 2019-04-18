#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::mem;
use std::ffi::CStr;

use winapi::um::libloaderapi;
use winapi::shared::{ntdef, minwindef, ntstatus};

use heim_common::prelude::*;
use heim_common::sys::windows::get_ntdll;

pub type SYSTEM_INFORMATION_CLASS = minwindef::DWORD;

pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = 8;

#[repr(C)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: ntdef::LARGE_INTEGER,
    pub KernelTime: ntdef::LARGE_INTEGER,
    pub UserTime: ntdef::LARGE_INTEGER,
    Reserved1: [ntdef::LARGE_INTEGER; 2],
    Reserved2: minwindef::ULONG,
}

pub unsafe fn NtQuerySystemInformation(
    SystemInformationClass: SYSTEM_INFORMATION_CLASS,
    SystemInformation: ntdef::PVOID,
    SystemInformationLength: minwindef::ULONG,
    ReturnLength: minwindef::PULONG,
) -> Result<()> {
    let ntdll = get_ntdll()?;

    let funcname = CStr::from_bytes_with_nul_unchecked(b"NtQuerySystemInformation\0");
    let func = libloaderapi::GetProcAddress(ntdll, funcname.as_ptr());

    if func.is_null() {
        return Err(Error::new(ErrorKind::Incompatible))
    }

    let func: extern "stdcall" fn(
        SYSTEM_INFORMATION_CLASS,
        ntdef::PVOID,
        minwindef::ULONG,
        minwindef::PULONG,
    ) -> ntdef::NTSTATUS = mem::transmute(func as *const ());

    let result = func(
        SystemInformationClass,
        SystemInformation,
        SystemInformationLength,
        ReturnLength,
    );

    if result == ntstatus::STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}
