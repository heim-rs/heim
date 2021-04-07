use std::marker::Unpin;
use std::path::Path;

use heim_common::prelude::{future, StreamExt, TryFutureExt};
use heim_runtime as rt;

use crate::Virtualization;

#[allow(clippy::unnecessary_wraps)] // TODO: Revisit later
fn try_guess_container(value: &str) -> Result<Virtualization, ()> {
    // TODO: Case-insensitive comparision?
    match value {
        "lxc" => Ok(Virtualization::Lxc),
        "lxc-libvirt" => Ok(Virtualization::LxcLibvirt),
        "systemd-nspawn" => Ok(Virtualization::SystemdNspawn),
        "docker" => Ok(Virtualization::Docker),
        "podman" => Ok(Virtualization::Podman),
        "rkt" => Ok(Virtualization::Rkt),
        "wsl" => Ok(Virtualization::Wsl),
        _ => Ok(Virtualization::Unknown),
    }
}

async fn detect_wsl<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let line = rt::fs::read_first_line(path).await.map_err(|_| ())?;

    match line {
        ref probe if probe.contains("Microsoft") => Ok(Virtualization::Wsl),
        ref probe if probe.contains("WSL") => Ok(Virtualization::Wsl),
        _ => Err(()),
    }
}

async fn detect_systemd_container<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    // systemd PID 1 might have dropped this information into a file in `/run`.
    // This is better than accessing `/proc/1/environ`,
    // since we don't need `CAP_SYS_PTRACE` for that.
    let line = rt::fs::read_first_line(path).await.map_err(|_| ())?;

    try_guess_container(&line)
}

async fn detect_cgroups<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    // TODO: Can be done in a blocking task completely
    let lines = rt::fs::read_lines(path).await.map_err(|_| ())?;
    futures::pin_mut!(lines);

    while let Some(line) = lines.next().await {
        match line {
            // TODO: Is it `lxc` or `lxc-libvirt` here?
            Ok(l) if l.contains("lxc") => return Ok(Virtualization::Lxc),
            Ok(l) if l.contains("docker") => return Ok(Virtualization::Docker),
            Ok(l) if l.contains("rkt") => return Ok(Virtualization::Rkt),
            _ => continue,
        }
    }

    Err(())
}

async fn detect_openvz() -> Result<Virtualization, ()> {
    // TODO: Can be done in a blocking task completely
    let f1 = rt::fs::path_exists(rt::linux::procfs_root().join("vz"));
    let f2 = rt::fs::path_exists(rt::linux::procfs_root().join("bc"));

    match futures::join!(f1, f2) {
        // `/proc/vz` exists in container and outside of the container,
        // `/proc/bc` only outside of the container.
        (true, false) => Ok(Virtualization::OpenVz),
        _ => Err(()),
    }
}

async fn detect_init_env<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    let contents = rt::fs::read_to_string(path).await.map_err(|_| ())?;

    let matched = contents
        .split('\0')
        .filter_map(|var| {
            let mut parts = var.split('=');
            // TODO: Should not it be a case-insensitive comparision?
            if let Some("container") = parts.next() {
                if let Some(value) = parts.next() {
                    return try_guess_container(value).ok();
                }
            }

            None
        })
        .next();

    match matched {
        Some(virt) => Ok(virt),
        None => Err(()),
    }
}

pub async fn detect_container() -> Result<Virtualization, ()> {
    future::err(())
        .or_else(|_| detect_openvz())
        .or_else(|_| detect_wsl(rt::linux::procfs_root().join("sys/kernel/osrelease")))
        .or_else(|_| detect_systemd_container("/run/systemd/container"))
        .or_else(|_| detect_init_env("/proc/1/environ"))
        // TODO: Check for a `/proc/1/environ` if there is `container` env var exists
        .or_else(|_| detect_cgroups("/proc/self/cgroup"))
        .await
}

#[cfg(test)]
mod tests {
    use super::{detect_init_env, detect_wsl};
    use std::io::Write;

    use crate::Virtualization;

    #[heim_derive::test]
    async fn test_wsl_1() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"Microsoft Windows Subsystem for Linux")
            .unwrap();
        let res = detect_wsl(f).await;

        assert_eq!(res, Ok(Virtualization::Wsl));
    }

    #[heim_derive::test]
    async fn test_wsl_2() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"Microsoft WSL").unwrap();
        let res = detect_wsl(f).await;

        assert_eq!(res, Ok(Virtualization::Wsl));
    }

    #[heim_derive::test]
    async fn test_init_env() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"LANG=C\0container=podman\0USER=root").unwrap();
        let res = detect_init_env(f).await;

        assert_eq!(res, Ok(Virtualization::Podman))
    }
}
