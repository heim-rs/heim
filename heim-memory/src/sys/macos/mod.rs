mod memory;
mod swap;
mod bindings;

lazy_static::lazy_static! {
    static ref PAGE_SIZE: u64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as u64
        }
    };
}

pub use self::memory::*;
pub use self::swap::*;
