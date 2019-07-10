# heim

[![Latest Version](https://img.shields.io/crates/v/heim.svg)](https://crates.io/crates/heim)
[![Latest Version](https://docs.rs/heim/badge.svg)](https://docs.rs/heim)
[![dependency status](https://deps.rs/crate/heim/0.0.3/status.svg)](https://deps.rs/crate/heim/0.0.3)
[![Build Status](https://dev.azure.com/heim-rs/heim/_apis/build/status/heim-rs.heim?branchName=master)](https://dev.azure.com/heim-rs/heim/_build/latest?definitionId=1&branchName=master)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-green.svg)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
[![Gitter](https://badges.gitter.im/heim-rs/heim.svg)](https://gitter.im/heim-rs/heim)

> Cross-platform framework for system information fetching

`heim` is an ongoing attempt to create the best tool for system information fetching
(ex., CPU, memory, disks or processes stats) in the Rust crates ecosystem.\
It targets to have at least the same functionality as
[psutil](https://github.com/giampaolo/psutil),
[gopsutil](https://github.com/shirou/gopsutil) or
[oshi](https://github.com/oshi/oshi) eventually.

## Technical notes

`heim` API targets to compile with stable Rust 1.36+;
but examples, tests and benchmarks are using [`async_await`](https://github.com/rust-lang/rust/issues/50547)
feature, and therefore, are requiring the `nightly`.\
Of course, you can use `heim` without the `async` keyword,
just be careful with the examples.

`heim` is using `futures = "0.3"` and it is expected from users to understand
how futures are working, how to use them and what differences there are
between `0.1` and `0.3` versions.\
It is not a project goal to provide any kind of information about how to
combine `heim` with `actix`, `hyper`, `tide` or any other Rust crate.

## Background

`heim` has few key things, which are defining its development and public interface:

 1. Async-first.\
    Async support in Rust becomes a first level citizen
    and it is about time to use it.
    While many things here are not requiring to be async right now,
    it will help to create better and faster implementations later.

 2. Cross-platform.\
    Any code from `heim` should just work at any platform supported.
    OS-specific things are exists, but forces user to pay attention to them
    due to API design.

 3. Smallest pieces possible.\
    Thanks to a various `futures` combinators, it's up to you
    to choose the exact information you want to get.

 4. Idiomatic and easy to use.
 
## Platform support

At the moment it is in **MVP** phase, which means that [Tier 1](https://forge.rust-lang.org/platform-support.html#tier-1)
platforms only (Linux, macOS and Windows for `i686` and `x86_64`)
are **partially** supported.
You may want to check the [Github projects page](https://github.com/heim-rs/heim/projects)
for more information.

Please, be aware, that at the moment `heim` (and all sub-crates)
has the "**experimental**" status,
so consider to double check the results before pushing your code to a production.

## Donations

If you appreciate my work and want to support me or speed up the project development,
you can do it [here](https://svartalf.info/donate/).

## License

Licensed under either of [Apache License 2.0](https://github.com/heim-rs/heim/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/heim-rs/heim/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
