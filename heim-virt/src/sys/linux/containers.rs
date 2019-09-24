use std::marker::Unpin;
use std::path::Path;

use heim_common::prelude::{future, Future, FutureExt, StreamExt, TryFutureExt, TryStreamExt};
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

fn detect_wsl<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    fs::read_first_line(path).map_err(|_| ()).and_then(|line| {
        let result = match line {
            ref probe if probe.contains("Microsoft") => Ok(Virtualization::Wsl),
            ref probe if probe.contains("WSL") => Ok(Virtualization::Wsl),
            _ => Err(()),
        };

        future::ready(result)
    })
}

fn detect_systemd_container<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    // systemd PID 1 might have dropped this information into a file in `/run`.
    // This is better than accessing `/proc/1/environ`,
    // since we don't need `CAP_SYS_PTRACE` for that.
    fs::read_first_line(path)
        .map_err(|_| ())
        .and_then(|line| future::ready(try_guess_container(&line)))
}

fn detect_cgroups<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    fs::read_lines(path)
        .map_err(|_| ())
        .try_filter_map(|line| {
            match () {
                // TODO: Is it `lxc` or `lxc-libvirt` here?
                _ if line.contains("lxc") => future::ok(Some(Virtualization::Lxc)),
                _ if line.contains("docker") => future::ok(Some(Virtualization::Docker)),
                _ if line.contains("machine-rkt") => future::ok(Some(Virtualization::Rkt)),
                _ => future::err(()),
            }
        })
        .into_future()
        .map(|(value, _)| match value {
            Some(Ok(virt)) => Ok(virt),
            _ => Err(()),
        })
}

fn detect_openvz() -> impl Future<Output = Result<Virtualization, ()>> {
    let f1 = fs::path_exists("/proc/vz");
    let f2 = fs::path_exists("/proc/bc");

    future::join(f1, f2).map(|result| {
        match result {
            // `/proc/vz` exists in container and outside of the container,
            // `/proc/bc` only outside of the container.
            (true, false) => Ok(Virtualization::OpenVz),
            _ => Err(()),
        }
    })
}

fn detect_init_env<T>(path: T) -> impl Future<Output = Result<Virtualization, ()>>
where
    T: AsRef<Path> + Send + Unpin + 'static,
{
    fs::read_to_string(path)
        .map_err(|_| ())
        .and_then(|contents| {
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
                Some(virt) => future::ok(virt),
                None => future::err(()),
            }
        })
}

pub fn detect_container() -> impl Future<Output = Result<Virtualization, ()>> {
    future::err(())
        .or_else(|_| detect_openvz())
        .or_else(|_| detect_wsl("/proc/sys/kernel/osrelease"))
        .or_else(|_| detect_systemd_container("/run/systemd/container"))
        .or_else(|_| detect_init_env("/proc/1/environ"))
        // TODO: Check for a `/proc/1/environ` if there is `container` env var exists
        .or_else(|_| detect_cgroups("/proc/self/cgroup"))
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
