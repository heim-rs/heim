cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub mod windows;
    }
}
