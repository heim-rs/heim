use std::mem;

use winapi::um::sysinfoapi;

pub mod power;
pub mod winternl;

// TODO: This one can be cached in the `lazy_static` / `once_cell`
pub fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();
    unsafe {
        // TODO: Should `GetNativeSystemInfo` be used here instead?
        sysinfoapi::GetSystemInfo(info.as_mut_ptr());

        info.assume_init()
    }
}
