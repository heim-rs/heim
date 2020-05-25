# heim

![Project banner](./.github/readme-logo.png)

[![Latest Version](https://img.shields.io/crates/v/heim.svg)](https://crates.io/crates/heim)
[![Latest Version](https://docs.rs/heim/badge.svg)](https://docs.rs/heim)
[![User guide](https://img.shields.io/badge/user%20guide-book-brightgreen)](https://heim-rs.github.io/book/)
[![Gitter](https://badges.gitter.im/heim-rs/heim.svg)](https://gitter.im/heim-rs/heim)\
[![Coverage Status](https://github.com/heim-rs/heim/workflows/Continuous%20integration/badge.svg)](https://github.com/heim-rs/heim/actions?workflow=Continuous+integration)
[![Financial Contributors on Open Collective](https://opencollective.com/heim-rs/all/badge.svg?label=financial+contributors)](https://opencollective.com/heim-rs)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

> Cross-platform library for system information fetching

`heim` is an ongoing attempt to create the best tool for system information fetching
(ex., CPU, memory, disks or processes stats) in the Rust crates ecosystem.\
It targets to have at least the same functionality as
[psutil](https://github.com/giampaolo/psutil),
[gopsutil](https://github.com/shirou/gopsutil) or
[oshi](https://github.com/oshi/oshi) eventually.

Check [user guide](https://heim-rs.github.io/book/) to get more information
on the `heim` goals, integrations and showcases.

Why should I use `heim` instead of *{crate-name}*?
See the [comparison](https://github.com/heim-rs/heim/blob/master/COMPARISON.md) page.

Examples can be found [here](https://github.com/heim-rs/heim/tree/master/examples).

## Background

`heim` has a few key goals which define its development and public interface:

 1. [Async-first](https://heim-rs.github.io/book/async/index.html)
    with [tokio](https://tokio.rs) and [async-std](https://async.rs) support.\
    Bundled polyfill option could be used for other use cases, see [documentation](https://docs.rs/heim)
    for more details.
 2. [Cross-platform](https://heim-rs.github.io/book/introduction/platforms.html) with [platform-specific extensions](https://heim-rs.github.io/book/api/platform-specific.html).
 3. Modular design.
 4. Idiomatic and easy to use.

## Technical notes

`heim` requires Rust 1.40 or higher; this version is explicitly tested in CI
and may be bumped in any major or minor release as needed.\
Any changes to the supported minimum version will be called out in the
[release notes](https://github.com/heim-rs/heim/blob/master/CHANGELOG.md).

## Platform support

Right now `heim` support [Tier 1](https://forge.rust-lang.org/platform-support.html#tier-1)
platforms (Linux, macOS, and Windows for `i686` and `x86_64`).
You can check the [GitHub projects page](https://github.com/heim-rs/heim/projects)
for more information.

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
