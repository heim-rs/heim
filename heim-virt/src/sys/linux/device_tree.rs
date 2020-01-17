use std::marker::Unpin;
use std::path::Path;

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
))]
use heim_common::prelude::TryFutureExt;
use heim_common::prelude::{future, Future, FutureExt, StreamExt, TryStreamExt};
use heim_runtime::fs;

use crate::Virtualization;

#[allow(unused)]
const DEVICE_TREE_ROOT: &str = "/proc/device-tree";

#[allow(unused)]
const HYPERVISOR_COMPAT_PATH: &str = "/proc/device-tree/hypervisor/compatible";

#[allow(unused)]
fn hypervisor<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    fs::read_lines(path)
        .into_stream() // TODO: Looks dumb
        .into_future()
        .map(|(try_line, _)| match try_line {
            Some(Ok(ref line)) if line == "linux,kvm" => Ok(Virtualization::Kvm),
            Some(Ok(ref line)) if line.contains("xen") => Ok(Virtualization::Xen),
            Some(Ok(..)) => Ok(Virtualization::Unknown),
            _ => Err(()),
        })
}

#[allow(unused)]
fn device_tree<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    fs::read_dir(path)
        .try_filter(|entry| {
            let matched = match entry.file_name().to_str() {
                Some(file_name) if file_name.contains("fw-cfg") => true,
                _ => false,
            };
            future::ready(matched)
        })
        .into_stream()
        .into_future()
        .map(|(res, _)| match res {
            Some(..) => Ok(Virtualization::Qemu),
            None => Err(()),
        })
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
))]
pub fn detect_vm_device_tree() -> impl Future<Output = Result<Virtualization, ()>> {
    hypervisor(HYPERVISOR_COMPAT_PATH).or_else(|_| device_tree(DEVICE_TREE_ROOT))
}

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
pub fn detect_vm_device_tree() -> impl Future<Output = Result<Virtualization, ()>> {
    future::err(())
}

#[cfg(test)]
mod tests {
    use super::hypervisor;
    use crate::Virtualization;
    use std::io::Write;

    #[heim_derive::test]
    async fn test_kvm() {
        let mut f = tempfile::NamedTempFile::new().unwrap();

        f.write_all(b"linux,kvm\nsome,other,stuff").unwrap();

        let result = hypervisor(f).await;

        assert_eq!(Ok(Virtualization::Kvm), result);
    }

    #[heim_derive::test]
    async fn test_xen() {
        let mut f = tempfile::NamedTempFile::new().unwrap();

        f.write_all(b"thereis,xen").unwrap();

        let result = hypervisor(f).await;

        assert_eq!(Ok(Virtualization::Xen), result);
    }

    #[heim_derive::test]
    async fn test_unknown() {
        let mut f = tempfile::NamedTempFile::new().unwrap();

        f.write_all(b"nes-emulator").unwrap();

        let result = hypervisor(f).await;

        assert_eq!(Ok(Virtualization::Unknown), result);
    }
}
