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
 * `cpu::os::unix::loadavg` function for load average values fetching
 * `net::Nic::is_running` method for checking network interface running state (#223)

### Changed

 * MSRV bumped to Rust 1.40.0+
 * Examples moved to the separate workspace crate
 * Benchmarks moved to the separate workspace crate
 * `process::Process::cwd` for Windows panics instead of returning blank error, as this method is not implemented yet
 * `process::Process::cwd` for Linux returns `AccessDenied` error if IO operation fails with the permission error (#226)
 * Internal blocking operations are grouped together as much as possible in order to reduce execution time
 * `net::Nic::is_up` method represents only "up" state now, `is_running` method added also (#223)

### Removed

 * Preliminary support for `tokio` and `async-std` crates introduced in `v0.1.0-alpha.1` was removed

### Fixed

 * `cpu::times` for Linux correctly parses `/proc/stat` (#233)

## Older versions

 * [`0.0.x` versions](https://github.com/heim-rs/heim/blob/v0.0.10/CHANGELOG.md)
