use std::ffi::CStr;
use std::io;
use std::mem;
use std::ptr;

/// Safer wrapper for `libc::sysctlbyname`
///
/// ## Safety
///
/// It is up to caller to provide proper null-terminated C string for `key` argument,
/// ex. `sysctlbyname(b"hw.cpufrequency\0")`.
pub unsafe fn sysctlbyname<T>(key: &[u8]) -> io::Result<T> {
    let key = CStr::from_bytes_with_nul_unchecked(key);
    let mut value = mem::MaybeUninit::<T>::uninit();
    let mut length = mem::size_of::<T>();

    let result = libc::sysctlbyname(
        key.as_ptr(),
        value.as_mut_ptr() as *mut libc::c_void,
        &mut length,
        ptr::null_mut(),
        0,
    );

    if result == 0 {
        Ok(value.assume_init())
    } else {
        // TODO: Attach error context
        Err(io::Error::last_os_error())
    }
}
