cfg_if::cfg_if! {
    if #[cfg(feature = "runtime-polyfill")] {
        mod polyfill;

        pub use self::polyfill::*;
    } else if #[cfg(feature = "runtime-async-std")] {
        compile_error!("`runtime-async-std` feature is a stub, see https://github.com/heim-rs/heim/issues/133");
    } else if #[cfg(feature = "runtime-tokio")] {
        compile_error!("`runtime-tokio` feature is a stub, see https://github.com/heim-rs/heim/issues/82");
    } else {
        compile_error!("You need to select the runtime");
    }
}
