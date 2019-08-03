# Comparing `heim` to other crates

This page provides an opinionated comparision between `heim` and 
other Rust crates with the similar functionality.

If you think that the results here wrong or outdated,
feel free to [create an issue](https://github.com/heim-rs/heim/issues/new)
or send a message [in the chat](https://gitter.im/heim-rs/heim).

## Crates used

* [`heim = "0.0.5"`](https://crates.io/crates/heim)
* [`psutil = "1.7.0"`](https://crates.io/crates/psutil)
* [`sysinfo = "0.9.1"`](https://crates.io/crates/sysinfo)
* [`sys-info = "0.5.7"`](https://crates.io/crates/sys-info)
* [`systemstat = "0.1.4"`](https://crates.io/crates/systemstat)

## Common

|                | heim       | psutil | sysinfo | sys-info | systemstat |
|----------------|------------|--------|---------|----------|------------|
| Pure Rust      | ✓          | ✓      | ✓       | ✗        | ✓          |
| Execution flow | async *1   | sync   | sync    | sync     | sync       |

> *1: partial, ex. https://github.com/heim-rs/heim/issues/56

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
| logical count  | ✓        | ✓      | ✗ *2    | ✓        | ✗ *2       |
| physical count | ✓        | ✓      | ✗ *2    | ✗ *2     | ✗ *2       |
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
| processes   | ✗        | ✓      | ✓       | ✗        | ✗          |

## Sensors information

|              | heim     | psutil | sysinfo | sys-info | systemstat |
|--------------|----------|--------|---------|----------|------------|
| temperatures | ✗        | ✗      | ✓       | ✗        | ✓          |
| fans         | ✗        | ✗      | ✗       | ✗        | ✗          |
