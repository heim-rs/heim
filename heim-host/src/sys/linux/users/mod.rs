cfg_if::cfg_if! {
    if #[cfg(target_env = "musl")] {
        mod musl;

        pub use self::musl::*;
    } else {
        mod other;

        pub use self::other::*;
    }
}
