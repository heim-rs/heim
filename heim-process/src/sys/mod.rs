mod linux;

pub use self::linux::*;

//cfg_if::cfg_if! {
//    if #[cfg(target_os = "linux")] {
//        mod linux;
//
//        pub use self::linux::*;
//    } else {
//        compile_error!("Unsupported target OS");
//    }
//}
