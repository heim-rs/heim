# heim-disk

This crate provides cross-platform functions to query information
about system disks.

It is a part of [heim project](https://github.com/heim-rs),
and preferably should not be used directly,
but via [heim](https://crates.io/crates/heim) crate.

## Compatibility matrix

|             | Linux | MacOS | Windows | FreeBSD | OpenBSD | Solaris | Redox |
| ----------- | ----- | ----- | ------- | ------- | ------- | ------- | ----- |
|             |       |       |         |         |         |         |       |
|             |       |       |         |         |         |         |       |

Legend:

 * `✓` — implemented
 * `≈` — implemented, but not reliable (e.g. might not support all targeted OS versions)
 * ` ` — not implemented
 * `X` — not available and will not be implemented
 * `?` — not sure if possible
