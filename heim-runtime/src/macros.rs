//! Weirdly-complicated way to re-export `futures::join!` and `futures::try_join!` macros
//! by using internal hidden APIs.
//!
//! TODO: Obviously bad solution and should be replaced with something better.

#[macro_export]
macro_rules! join {
    ($($tokens:tt)*) => {
        $crate::futures_util::join! {
            futures_crate_path ( ::heim_runtime::futures_util )
            $( $tokens )*
        }
    }
}

#[macro_export]
macro_rules! try_join {
    ($($tokens:tt)*) => {
        $crate::futures_util::try_join! {
            futures_crate_path ( ::heim_runtime::futures_util )
            $( $tokens )*
        }
    }
}
