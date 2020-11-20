#![cfg(target_os = "linux")]

use rusty_fork::rusty_fork_test;
use std::path::{Path, PathBuf};

use heim_runtime as rt;

rusty_fork_test! {
    #[test]
    fn test_default_proc_root() {
        assert_eq!(rt::linux::procfs_root(), PathBuf::from("/proc"));
    }
}

rusty_fork_test! {
    #[test]
    fn test_nonstandard_proc_root() {
        rt::linux::set_procfs_root(Path::new("/host/proc"));
        assert_eq!(rt::linux::procfs_root(), PathBuf::from("/host/proc"));
    }
}

rusty_fork_test! {
    #[test]
    fn test_default_sys_root() {
        assert_eq!(rt::linux::sysfs_root(), PathBuf::from("/sys"));
    }
}

rusty_fork_test! {
    #[test]
    fn test_nonstandard_sys_root() {
        rt::linux::set_sysfs_root(Path::new("/host/sys"));
        assert_eq!(rt::linux::sysfs_root(), PathBuf::from("/host/sys"));
    }
}
