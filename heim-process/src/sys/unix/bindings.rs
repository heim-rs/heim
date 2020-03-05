/// Copied from the Rust sources.
///
/// Reference: https://github.com/rust-lang/rust/blob/75cf41afb468152611212271bae026948cd3ba46/src/libstd/sys/unix/os.rs
extern "C" {
    #[cfg(not(target_os = "dragonfly"))]
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "freebsd"),
        link_name = "__error"
    )]
    #[cfg_attr(
        any(
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "bitrig",
            target_os = "android"
        ),
        link_name = "__errno"
    )]
    #[cfg_attr(target_os = "solaris", link_name = "___errno")]
    #[cfg_attr(target_os = "linux", link_name = "__errno_location")]
    pub fn errno_location() -> *mut libc::c_int;
}

#[allow(trivial_numeric_casts)]
pub fn errno() -> i32 {
    unsafe { (*errno_location()) as i32 }
}

/// Sets the platform-specific value of errno
#[allow(trivial_numeric_casts)]
pub fn set_errno(e: i32) {
    unsafe { *errno_location() = e as libc::c_int }
}
