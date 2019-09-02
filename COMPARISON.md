# Comparing `heim` to other crates

This page provides an opinionated comparison between `heim` and
other Rust crates with the similar functionality.\
Primary goals are to understand what pieces are missing in `heim`
and to help users choose the crate suitable for their needs.

If you think that the results here are wrong or outdated,
feel free to [create an issue](https://github.com/heim-rs/heim/issues/new)
or send a message [in the chat](https://gitter.im/heim-rs/heim).

## Crates used

* [`heim = "0.0.7"`](https://crates.io/crates/heim)
* [`psutil = "1.7.0"`](https://crates.io/crates/psutil)
* [`sysinfo = "0.9.2"`](https://crates.io/crates/sysinfo)
* [`sys-info = "0.5.7"`](https://crates.io/crates/sys-info)
* [`systemstat = "0.1.4"`](https://crates.io/crates/systemstat)

## Basics

|                | heim             | psutil | sysinfo | sys-info | systemstat |
|----------------|------------------|--------|---------|----------|------------|
| Pure Rust      | ✓                | ✓      | ✓       | ✗        | ✓          |
| Execution flow | async            | sync   | sync    | sync     | sync       |
| License        | Apache 2.0 / MIT | MIT    | MIT     | MIT      | Unlicense  |

## Supported platforms

|            | heim     | psutil | sysinfo | sys-info | systemstat |
|------------|----------|--------|---------|----------|------------|
| Linux      | ✓        | ✓      | ✓       | ✓        | ✓          |
| macOS      | ✓        | ✗      | ✓       | ✓        | ✓          |
| Windows    | ✓        | ✗      | ✓       | ✓        | ✓          |
| FreeBSD    | ✗        | ✗      | ✗       | ✗        | ✓          |
| OpenBSD    | ✗        | ✗      | ✗       | ✗        | ✓          |
| Android    | ✗        | ✗      | ?       | ✗        | ✗          |

## CPU information

|                | heim     | psutil | sysinfo | sys-info | systemstat |
|----------------|----------|--------|---------|----------|------------|
| logical count  | ✓        | ✓      | ✗       | ✓        | ✗          |
| physical count | ✓        | ✓      | ✗       | ✗        | ✗          |
| frequency      | ✓        | ✗      | ✗       | ✓        | ✗          |
| time           | ✓        | ✓      | ✗       | ✗        | ✓          |
| stats          | ✓        | ✗      | ✗       | ✗        | ✗          |

## Disks information

|                | heim     | psutil | sysinfo | sys-info | systemstat |
|----------------|----------|--------|---------|----------|------------|
| usage          | ✓        | ✓      | ✓       | ✓        | ✓          |
| partitions     | ✓        | ✓      | ✓       | ✗        | ✓          |
| IO counters    | ✓        | ✓      | ✗       | ✗        | ✓          |

## Host information

|                  | heim     | psutil | sysinfo | sys-info | systemstat |
|------------------|----------|--------|---------|----------|------------|
| uptime           | ✓        | ✓      | ✓       | ✗        | ✓          |
| boot time        | ✗        | ✗      | ✗       | ✓        | ✓          |
| load average     | ✗        | ✓      | ?       | ✓        | ✓          |
| name/version     | ✓        | ✗      | ✗       | ✓        | ✗          |
| hostname         | ✓        | ✗      | ✗       | ✓        | ✗          |
| users            | ✓        | ✗      | ✗       | ✗        | ✗          |

## Memory information

|         | heim     | psutil | sysinfo | sys-info | systemstat |
|---------|----------|--------|---------|----------|------------|
| memory  | ✓        | ✓      | ✓       | ✓        | ✓          |
| swap    | ✓        | ✓      | ✓       | ✓        | ✓          |

## Network information

|             | heim     | psutil | sysinfo | sys-info | systemstat |
|-------------|----------|--------|---------|----------|------------|
| NIC info    | ✓        | ✗      | ✗       | ✗        | ✓          |
| IO counters | ✓        | ✓      | ✓       | ✗        | ✓          |
| connections | ✗        | ✗      | ✗       | ✗        | ✗          |

## Processes information

|             | heim     | psutil | sysinfo | sys-info | systemstat |
|-------------|----------|--------|---------|----------|------------|
| pids        | ✓        | ✓      | ✓       | ✗        | ✗          |
| pid_exists  | ✓        | ✓      | ✓       | ✗        | ✗          |
| processes   | ✓        | ✓      | ✓       | ✗        | ✗          |

## Sensors information

|              | heim     | psutil | sysinfo | sys-info | systemstat |
|--------------|----------|--------|---------|----------|------------|
| temperatures | ✗        | ✗      | ✓       | ✗        | ✓          |
| fans         | ✗        | ✗      | ✗       | ✗        | ✗          |
