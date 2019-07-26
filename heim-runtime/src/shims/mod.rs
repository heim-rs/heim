cfg_if::cfg_if! {
    if #[cfg(feature = "reactor-polyfill")] {
        mod sync;

        pub use self::sync::*;
    }
}
