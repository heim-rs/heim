//! Windows-specific routines used across `heim` crates.

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::ffi::CStr;
use std::ffi::OsStr;
use std::iter;
use std::mem;
use std::os::windows::ffi::OsStrExt;

use winapi::shared::{minwindef, ntdef};
use winapi::um::{libloaderapi, winnt};

use crate::prelude::*;

mod time;

#[allow(missing_docs)]
pub type SYSTEM_INFORMATION_CLASS = minwindef::DWORD;

#[allow(missing_docs)]
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = 2;
#[allow(missing_docs)]
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = 5;
#[allow(missing_docs)]
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = 8;
#[allow(missing_docs)]
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = 23;

//typedef enum _SYSTEM_INFORMATION_CLASS {
//    SystemBasicInformation = 0,
//    SystemPerformanceInformation = 2,
//    SystemTimeOfDayInformation = 3,
//    SystemProcessInformation = 5,
//    SystemProcessorPerformanceInformation = 8,
//    SystemInterruptInformation = 23,
//    SystemExceptionInformation = 33,
//    SystemRegistryQuotaInformation = 37,
//    SystemLookasideInformation = 45
//} SYSTEM_INFORMATION_CLASS;

/// `heim` is using some private functions from the `ntdll.dll` file at the moment,
/// and since it is not possible to link with it,
/// we are required to do the run-time dynamic linking
///
/// https://docs.microsoft.com/ru-ru/windows/desktop/Dlls/using-run-time-dynamic-linking
/// https://docs.microsoft.com/ru-ru/windows/desktop/api/libloaderapi/nf-libloaderapi-getmodulehandlew
///
/// ## Returns
///
/// *Pointer* to the loaded `ntdll.dll` library
pub unsafe fn get_ntdll() -> Result<minwindef::HMODULE> {
    let dll_wide: Vec<winnt::WCHAR> = OsStr::new("ntdll.dll")
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    let module = libloaderapi::GetModuleHandleW(dll_wide.as_ptr());
    if module.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(module)
    }
}

/// Querying some shady and undocumented Windows APIs, what could even go wrong?
pub unsafe fn NtQuerySystemInformation(
    SystemInformationClass: SYSTEM_INFORMATION_CLASS,
    SystemInformation: ntdef::PVOID,
    SystemInformationLength: minwindef::ULONG,
    ReturnLength: minwindef::PULONG,
) -> Result<ntdef::NTSTATUS> {
    let ntdll = get_ntdll()?;

    let funcname = CStr::from_bytes_with_nul_unchecked(b"NtQuerySystemInformation\0");
    let func = libloaderapi::GetProcAddress(ntdll, funcname.as_ptr());

    if func.is_null() {
        return Err(Error::last_os_error());
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

    Ok(result)
}
