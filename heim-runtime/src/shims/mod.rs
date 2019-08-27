mod polyfill;

pub use self::polyfill::*;

//cfg_if::cfg_if! {
//    if #[cfg(feature = "runtime-polyfill")] {
//        mod polyfill;
//
//        pub use self::polyfill::*;
//    }
//}
