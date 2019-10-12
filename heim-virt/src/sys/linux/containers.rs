use std::path::Path;

use heim_common::prelude::{future, StreamExt, TryFutureExt};
use heim_runtime::fs;

use crate::Virtualization;

fn try_guess_container(value: &str) -> Result<Virtualization, ()> {
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
    T: AsRef<Path>,
{
    let line = fs::read_first_line(path).map_err(|_| ()).await?;
    match &line {
        probe if probe.contains("Microsoft") => Ok(Virtualization::Wsl),
        probe if probe.contains("WSL") => Ok(Virtualization::Wsl),
        _ => Err(()),
    }
}

async fn detect_systemd_container<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path>,
{
    // systemd PID 1 might have dropped this information into a file in `/run`.
    // This is better than accessing `/proc/1/environ`,
    // since we don't need `CAP_SYS_PTRACE` for that.
    let line = fs::read_first_line(path).map_err(|_| ()).await?;

    try_guess_container(&line)
}

async fn detect_cgroups<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path>,
{
    let mut lines = fs::read_lines(path).map_err(|_| ()).await?;
    while let Some(try_line) = lines.next().await {
        match try_line {
            Ok(line) if line.contains("lxc") => return Ok(Virtualization::Lxc),
            Ok(line) if line.contains("docker") => return Ok(Virtualization::Docker),
            Ok(line) if line.contains("machine-rkt") => return Ok(Virtualization::Rkt),
            _ => continue,
        }
    }

    Err(())
}

async fn detect_openvz() -> Result<Virtualization, ()> {
    let f1 = fs::path_exists("/proc/vz");
    let f2 = fs::path_exists("/proc/bc");

    let (vz, bc) = future::join(f1, f2).await;
    match (vz, bc) {
        // `/proc/vz` exists in container and outside of the container,
        // `/proc/bc` only outside of the container.
        (true, false) => Ok(Virtualization::OpenVz),
        _ => Err(()),
    }
}

async fn detect_init_env<T>(path: T) -> Result<Virtualization, ()>
where
    T: AsRef<Path>,
{
    let contents = fs::read_to_string(path).map_err(|_| ()).await?;
    for part in contents.split('\0') {
        let mut parts = part.split('=');
        // TODO: Should not it be a case-insensitive comparision?
        if let Some("container") = parts.next() {
            if let Some(value) = parts.next() {
                return try_guess_container(value);
            }
        }
    }

    Err(())
}

pub async fn detect_container() -> Result<Virtualization, ()> {
    future::err(())
        .or_else(|_| detect_openvz())
        .or_else(|_| detect_wsl("/proc/sys/kernel/osrelease"))
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
