# heim

![Project banner](./.github/readme-logo.png)

[![Latest Version](https://img.shields.io/crates/v/heim.svg)](https://crates.io/crates/heim)
[![Latest Version](https://docs.rs/heim/badge.svg)](https://docs.rs/heim)
[![dependency status](https://deps.rs/crate/heim/0.0.9/status.svg)](https://deps.rs/crate/heim/0.0.9)
[![Coverage Status](https://github.com/heim-rs/heim/workflows/Continuous%20integration/badge.svg)](https://github.com/heim-rs/heim/actions?workflow=Continuous+integration)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-green.svg)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)
[![Gitter](https://badges.gitter.im/heim-rs/heim.svg)](https://gitter.im/heim-rs/heim)
![Platforms supported](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-brightgreen)
[![Financial Contributors on Open Collective](https://opencollective.com/heim-rs/all/badge.svg?label=financial+contributors)](https://opencollective.com/heim-rs)

> Cross-platform library for system information fetching

`heim` is an ongoing attempt to create the best tool for system information fetching
(ex., CPU, memory, disks or processes stats) in the Rust crates ecosystem.\
It targets to have at least the same functionality as
[psutil](https://github.com/giampaolo/psutil),
[gopsutil](https://github.com/shirou/gopsutil) or
[oshi](https://github.com/oshi/oshi) eventually.

Why should I use `heim` instead of *{crate-name}*?
See the [comparison](https://github.com/heim-rs/heim/blob/master/COMPARISON.md) page.

## Background

`heim` has a few key goals which define its development and public interface:

 1. Async-first.\
    Async support in Rust has become a first class citizen
    and it's about time to use it.
    While many things here do not require async right now,
    it will help create better and faster implementations later.

 2. Cross-platform.\
    Any code from `heim` should just work on any supported platforms.
    OS-specific things do exist, but the API design forces users to
    pay attention to them.

 3. Modular design.\
    Thanks to the various `futures` combinators, it's up to you
    to choose the exact information you want to get.

 4. Idiomatic and easy to use.

## Technical notes

`heim` requires stable Rust 1.36+,
but examples, tests and benchmarks use the [`async_await`](https://github.com/rust-lang/rust/issues/50547)
feature and therefore require a `nightly` compiler.\
Of course, you can use `heim` without the `async` keyword,
just be aware that it is used by the examples.

`heim` is using `std::future::Future` and it is expected that users understand
how futures work, how to use them, and what differences there are
between futures versions `0.1` and `0.3/std::future::Future`.\
It is not a project goal to provide any kind of information about how to
combine `heim` with `actix`, `hyper`, `tide`, or any other Rust crate.

At the moment, `heim` is async runtime agnostic, but in the future it may need to depend on runtime
specific features. Note that if this becomes the case, it is likely that the required runtime(s) will be
compatible with the [runtime](https://crates.io/crates/runtime) crate.
Users may want to consider this fact during development. See [#75](https://github.com/heim-rs/heim/issues/75) for more info.

## Platform support

At the moment, `heim` is in **MVP** phase, which means that there is only only **partial** support
for [Tier 1](https://forge.rust-lang.org/platform-support.html#tier-1)
platforms (Linux, macOS, and Windows for `i686` and `x86_64`).
You can check the [GitHub projects page](https://github.com/heim-rs/heim/projects)
for more information.

Please be aware that at the moment, `heim` (and all sub-crates)
has the "**experimental**" status,
so consider double checking the results before pushing your code to production.

## License

Licensed under either of [Apache License 2.0](https://github.com/heim-rs/heim/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/heim-rs/heim/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Donations

If you appreciate my work and want to support me or speed up the project development,
you can do it [here](https://svartalf.info/donate/) or
support this project at [Open Collective](https://opencollective.com/heim-rs).

## Contributors

### Code Contributors

This project exists thanks to all the people who [contribute](CONTRIBUTE.md).
<a href="https://github.com/heim-rs/heim/graphs/contributors"><img src="https://opencollective.com/heim-rs/contributors.svg?width=890&button=false" /></a>

### Financial Contributors

[Become](https://opencollective.com/heim-rs/contribute) a financial contributor and help us sustain our community.

#### Individuals

<a href="https://opencollective.com/heim-rs"><img src="https://opencollective.com/heim-rs/individuals.svg?width=890"></a>

#### Organizations

[Support this project](https://opencollective.com/heim-rs/contribute) with your organization. Your logo will show up here with a link to your website.
