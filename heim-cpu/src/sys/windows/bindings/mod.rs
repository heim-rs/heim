use std::mem;

use winapi::um::sysinfoapi;

pub mod power;
pub mod winternl;

// TODO: This one can be cached in the `lazy_static`
pub unsafe fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();
    sysinfoapi::GetSystemInfo(info.as_mut_ptr());

    info.assume_init()
}
