#![cfg(target_os = "linux")]

//! Linux-specific extension for configuring custom paths for Procfs
/// and Sysfs.
use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// Holds global state for custom paths. Static's leveraging this type
/// can only be set ONCE per binary run.
type RootCell = OnceCell<PathBuf>;

static PROCFS_ROOT: RootCell = OnceCell::new();
static SYSFS_ROOT: RootCell = OnceCell::new();

/// Instantiates the OnceCell holding PROCFS_ROOT in the case that it hasn't
/// already been instantiated.
///
/// This can only be set ONCE per binary run.
pub fn set_procfs_root<T: Into<Cow<'static, Path>>>(root: T) {
    let root = root.into().into_owned();
    let _ = PROCFS_ROOT.get_or_init(|| root);
}

/// Returns the static `Path` value of a configured PROCFS_ROOT.
///
/// If uninitialized, initializes the PROCFS_ROOT with the default path - `/proc`
pub fn procfs_root() -> &'static Path {
    PROCFS_ROOT.get_or_init(|| PathBuf::from("/proc")).as_ref()
}

/// Instantiates the OnceCell holding SYSFS_ROOT in the case that it hasn't
/// already been instantiated.
///
/// This can only be set ONCE per binary run.
pub fn set_sysfs_root<T: Into<Cow<'static, Path>>>(root: T) {
    let root = root.into().into_owned();
    let _ = SYSFS_ROOT.get_or_init(|| root);
}

/// Returns the static `Path` value of a configured PROCFS_ROOT.
///
/// If uninitialized, initializes the PROCFS_ROOT with the default path - `/sys`
pub fn sysfs_root() -> &'static Path {
    SYSFS_ROOT.get_or_init(|| PathBuf::from("/sys")).as_ref()
}
