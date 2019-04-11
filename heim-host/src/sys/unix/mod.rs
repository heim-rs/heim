use std::ffi::CStr;
use std::borrow::Cow;

mod platform;
mod users;

pub use self::platform::*;
pub use self::users::*;

// moo!
#[inline(always)]
pub(crate) unsafe fn into_cow(chars: &[libc::c_char]) -> Cow<str> {
    CStr::from_ptr(chars.as_ptr()).to_string_lossy()
}
