use winapi::shared::minwindef;

pub mod wtsapi32;

// TODO: Is not declared in `winapi` crate
// See https://github.com/retep998/winapi-rs/issues/780
pub const MAX_COMPUTERNAME_LENGTH: minwindef::DWORD = 31;
