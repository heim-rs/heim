use std::iter;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::um::{winnt, libloaderapi};
use winapi::shared::minwindef;

use crate::prelude::*;

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
