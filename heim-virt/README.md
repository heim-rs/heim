# heim-virt

This crate provides cross-platform functions to detect
virtualization system in which program is running.

At the moment not all declared virtualization systems are detected,
therefore this crate should be used very carefully.

See the [issues list](https://github.com/heim-rs/heim/issues?q=is%3Aissue+is%3Aopen+label%3AA-virt)
for a not supported currently systems.


It is a part of [heim project](https://github.com/heim-rs),
and preferably should not be used directly,
but via [heim](https://crates.io/crates/heim) crate.

## Compatibility matrix

|             | Linux | MacOS | Windows | FreeBSD | OpenBSD | Solaris | Redox |
| ----------- | ----- | ----- | ------- | ------- | ------- | ------- | ----- |
| detect      | ≈     |       |         |         |         |         |       |

Legend:

 * `✓` — implemented
 * `≈` — implemented, but not reliable (e.g. might not support all targeted OS versions)
 * ` ` — not implemented
 * `X` — not available and will not be implemented
 * `?` — not sure if possible
