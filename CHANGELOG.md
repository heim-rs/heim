# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This document describes changes for `0.1.x` versions,
refer to "[Older versions](#older-versions)" section
for information about previous releases.

## [Unreleased]

### Added

 * Integration with `tokio` async runtime
 * Integration with `async-std` async runtime
 * Polyfill async runtime for other use cases
 * `heim::Error` type contains cause data for debugging purposes now

### Changed

 * MSRV bumped to Rust 1.39+
 * Examples moved to the separate workspace crate
 * Benchmarks moved to the separate workspace crate
 * `Process::cwd` for Windows panics instead of returning blank error, as this method is not implemented yet
 * Huge internal refactoring across all sub-crates

## Older versions

 * [`0.0.x` versions](https://github.com/heim-rs/heim/blob/v0.0.10/CHANGELOG.md)
