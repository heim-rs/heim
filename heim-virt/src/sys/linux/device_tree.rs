use std::marker::Unpin;
use std::path::Path;

use heim_common::prelude::StreamExt;
#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
))]
use heim_common::prelude::TryFutureExt;
use heim_runtime as rt;

use crate::Virtualization;

#[allow(unused)]
const DEVICE_TREE_ROOT: &str = "/proc/device-tree";

#[allow(unused)]
const HYPERVISOR_COMPAT_PATH: &str = "/proc/device-tree/hypervisor/compatible";

#[allow(unused)]
async fn hypervisor<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let line = rt::fs::read_first_line(path).await.map_err(|_| ())?;
    match &line {
        l if l == "linux,kvm" => Ok(Virtualization::Kvm),
        l if l.contains("xen") => Ok(Virtualization::Xen),
        _ => Ok(Virtualization::Unknown),
    }
}

#[allow(unused)]
async fn device_tree<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let mut entries = rt::fs::read_dir(path).await.map_err(|_| ())?;
    while let Some(entry) = entries.next().await {
        let entry = entry.map_err(|_| ())?;

        match entry.file_name().to_str() {
            Some(file_name) if file_name.contains("fw-cfg") => return Ok(Virtualization::Qemu),
            _ => continue,
        }
    }

    Err(())
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
))]
pub async fn detect_vm_device_tree() -> Result<Virtualization, ()> {
    hypervisor(HYPERVISOR_COMPAT_PATH)
        .or_else(|_| device_tree(DEVICE_TREE_ROOT))
        .await
}

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
pub async fn detect_vm_device_tree() -> Result<Virtualization, ()> {
    Err(())
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
