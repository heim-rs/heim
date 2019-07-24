# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.5] - 2019-07-24

### Added

- `host::Platform::hostname` method for *nix ([#71](https://github.com/heim-rs/heim/issues/71)) and Windows ([#72](https://github.com/heim-rs/heim/issues/72))
- Extension traits with extra data for `host::User` for Linux ([#69](https://github.com/heim-rs/heim/issues/69)) and macOS ([#70](https://github.com/heim-rs/heim/issues/70))
- Extension traits with extra data for `net::Nic` for Linux and macOS ([#66](https://github.com/heim-rs/heim/issues/66))
- Support for `exFAT`, `f2fs`, `Hfs+`, `JFS`, `Reiser4`, `Btrfs`, `Minix`, `NILFS` and `XFS` filesystems for `disk::Filesystem` enum ([#61](https://github.com/heim-rs/heim/issues/61))

### Fixed

- `cpu::CpuFreq` for Linux was reporting frequencies in KHz instead of Hz ([#68](https://github.com/heim-rs/heim/issues/68))
- `disk::usage` was reporting `free` value incorrectly for Windows ([#73](https://github.com/heim-rs/heim/issues/73))

### Changed

- `cpu::CpuStats::soft_interrupts` method was moved out into the extension traits ([#57](https://github.com/heim-rs/heim/issues/57))
- `net::IoCounters::drop_sent` method was moved out into the extension traits ([#67](https://github.com/heim-rs/heim/issues/67))
- `disk::FileSystem::Reiserfs` enum member was renamed into the `Reiser3`

### Security

- Fix memory leak for `disk::usage` for *nix systems ([#77](https://github.com/heim-rs/heim/issues/77))
- Fix possible heap buffer overflow in `cpu::times` implementation for macOS ([#78](https://github.com/heim-rs/heim/issues/78))
- Fix possible heap buffer overflow in `net::io_counters` implementation for macOS ([#79](https://github.com/heim-rs/heim/issues/79))

## [0.0.4] - 2019-07-13

### Added

- First public version
