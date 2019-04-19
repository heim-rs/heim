mod memory;
mod swap;
mod bindings;

lazy_static::lazy_static! {
    static ref PAGE_SIZE: usize = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as usize
        }
    };
}

pub use self::memory::*;
pub use self::swap::*;
