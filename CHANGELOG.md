# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.10] - 2020-02-15

### Fixed

- Possible heap corruption error on Windows when calling `heim::cpu::physical_count` (#200)

### Changed

- `hex` dependency version frozen, new patch version broke the MSRV compatibility
- `heim-derive` crate is not used as a normal dependency for other workspace crates, should reduce compilation time

## [0.0.10] - 2019-11-22

### Added

- Preliminary support for ARM platforms

### Changed

- Update to `futures = "^0.3"` version
- Workaround for Rust nightly bug (#182)

## [0.0.8] - 2019-10-03

### Added

- `host::boot_time` function (#147, #148, #149)
- `process::Process::create_time` method (#100, #101, #102)
- `process::Process::is_running` method (#151, #152, #153)
- `process::Process::command` method (#97, #98)
- `process::Process::kill` method (#158, #159)
- `process::Process::terminate` method (#162, #163)
- `process::Process::suspend` method (#164, #165)
- `process::Process::resume` method (#164, #166)
- `APFS` support for `disk::FileSystem` (#171)
- Legacy systems support for `cpu::logical_count` for Linux (#58)

### Changed

- Truncated `process::Process::name` results are expanded for Linux (#154)

### Fixed

- `process::Process::exe`, `process::Process::cpu_time` and `process::Process::memory` for Windows
    are properly returning `AccessDenied` error in case of permission issues
- `process::Process::memory` and `process::Process::cpu_time` for macOS
    are properly returning `AccessDenied` error in case of permission issues

## [0.0.7] - 2019-08-30

### Added

- `sensors::temperatures` implementation for Linux (#85)
- `process::Process::current` for current process fetching (#117, #118, #119)
- `process::Process::get` for process fetching by provided pid (#117, #118, #119)
- `process::Process::io_counters` Linux-specific method returning process I/O counters (#127)
- `process::Process::net_io_counters` Linux-specific method returning process network I/O counters (#124)
- `process::Process::cpu_time` method returning CPU times for process (#107, #108, #109)
- `process::Process::memory` method returning process memory usage (#121, #122, #123)
- `process::Process::cpu_usage` method returning CPU usage by process (#134, #135, #136)
- `process::Process::cwd` for Linux and macOS (#103, #104)

### Changed

- `uom` crate is used for typed quantities used in API (#95)
- Replace `glob` crate usage from `cpu::frequency` for Linux with async fs shim
- Update to `futures-preview = "0.3.0-alpha.18"` version

## [0.0.6] - 2019-08-08

### Added

- `heim-runtime` crate with shims for async runtimes (only "sync" polyfill available at the moment)
- `process::processes` function returning stream of `process::Process`
- `process::Process` struct with `pid`, `parent_pid`, `name` and `exe` methods

### Fixed

- `disk::partitions` returned inconsistent data for Windows (#92)
- `disk::io_counters` failed on any empty removable drive in Windows (#94)

### Changed

- `cpu::os::linux::CpuTimeExt::steal` returns `Time` now instead of `Option<Time>` (#81)

## [0.0.5] - 2019-07-24

### Added

- `host::Platform::hostname` method for *nix (#71) and Windows (#72)
- Extension traits with extra data for `host::User` for Linux (#69) and macOS (#70)
- Extension traits with extra data for `net::Nic` for Linux and macOS (#66)
- Support for `exFAT`, `f2fs`, `Hfs+`, `JFS`, `Reiser4`, `Btrfs`, `Minix`, `NILFS` and `XFS` filesystems for `disk::Filesystem` enum (#61)

### Fixed

- `cpu::CpuFreq` for Linux was reporting frequencies in KHz instead of Hz (#68)
- `disk::usage` was reporting `free` value incorrectly for Windows (#73)

### Changed

- `cpu::CpuStats::soft_interrupts` method was moved out into the extension traits (#57)
- `net::IoCounters::drop_sent` method was moved out into the extension traits (#67)
- `disk::FileSystem::Reiserfs` enum member was renamed into the `Reiser3`

### Security

- Fix memory leak for `disk::usage` for *nix systems (#77)
- Fix possible heap buffer overflow in `cpu::times` implementation for macOS (#78)
- Fix possible heap buffer overflow in `net::io_counters` implementation for macOS (#79)

## [0.0.4] - 2019-07-13

### Added

- First public version
