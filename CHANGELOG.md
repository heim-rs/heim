# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This document describes changes for `0.1.x` versions,
refer to "[Older versions](#older-versions)" section
for information about previous releases.

## [Unreleased]

### Added

 * Async and blocking operations are handled by `smol` crate internally
 * `heim::Error` type contains cause data for debugging purposes now
 * `process::Process::priority` method for Windows (#217)
 * `process::Process::niceness` method for *nixes (#216)
 * `process::Process::wait` method for Linux and macOS (#213, #214)
 * `process::Process::environment` method for Linux and macOS (#208, #209)
 * `process::Process::cwd` for Windows, first unstable version (#267)
 * `cpu::os::unix::loadavg` function for load average values fetching
 * `net::Nic::is_running` method for checking network interface running state (#223)
 * `disk::Partition::usage` method to fetch disk usage information (#288)
 * ARM64 architecture detected now, as in Apple Silicon chips (#303)
 * "Windows Domain Controller" is detected by `heim::host::platform` for Windows (#302)
 * Windows domain name is provided by `heim::host::os::windows::PlatformExt` extension trait (#302)

### Changed

 * MSRV bumped to Rust 1.46.0+
 * Dependencies are now set with caret dependencies instead of tilde ones
 * Examples moved to the separate workspace crate
 * Benchmarks moved to the separate workspace crate
 * `process::Process::cwd` for Linux returns `AccessDenied` error if IO operation fails with the permission error (#226)
 * Internal blocking operations are grouped together as much as possible in order to reduce execution time
 * `net::Nic::is_up` method represents only "up" state now, `is_running` method added also (#223)
 * `heim::net::nic` returns `Send + Sync` `Stream` now (#313)

### Removed

 * Preliminary support for `tokio` and `async-std` crates introduced in `v0.1.0-alpha.1` was removed
 * Leaking `impl FromStr` and `impl TryFrom<char>` was removed for `process::Status` enum (#260)

### Fixed

 * `cpu::times` for Linux correctly parses `/proc/stat` (#233)
 * Handle addition overflow when calculating CPU interrupts stats for Windows (#250)
 * Swap memory calculation fixed for Windows (#307)
 * `heim::host::Platform::hostname` value is not truncated for Windows (#302)

## Older versions

 * [`0.0.x` versions](https://github.com/heim-rs/heim/blob/v0.0.10/CHANGELOG.md)
