# heim-cpu

This crate provides cross-platform functions to query information
about system CPUs.

It is a part of [heim project](https://github.com/heim-rs),
and preferably should not be used directly,
but via [heim](https://crates.io/crates/heim) crate.

## Compatibility matrix

|             | Linux | MacOS | Windows | FreeBSD | OpenBSD | Solaris | Redox |
| ----------- | ----- | ----- | ------- | ------- | ------- | ------- | ----- |
| time        | ✓     |       |         |         |         |         |       |
| times       | ✓     |       |         |         |         |         |       |
| stats       | ✓     |       |         |         |         |         |       |
| frequency   | ≈     |       |         |         |         |         |       |
| frequencies | ≈     | X     | X       | X       | X       | X       | ?     |

Legend:

 * `✓` — implemented
 * `≈` — implemented, but not reliable (e.g. might not support all OS versions)
 * ` ` — not implemented
 * `X` — not available and will not be implemented
 * `?` — not sure if possible
