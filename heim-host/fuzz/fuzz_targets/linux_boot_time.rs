#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate heim_host;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = heim_host::sys::linux::boot_time::parse(&s);
    }
});
