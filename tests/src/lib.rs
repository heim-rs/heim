use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

mod macros;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    }
}

pub mod prelude {
    pub use crate::assert_delta_le;
    pub use approx::*;
    pub use claim::*;
}
