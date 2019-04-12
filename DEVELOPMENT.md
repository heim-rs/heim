## Short intro about how things should be done

Consider we want to expose information about CPU statistics (context switches and interrupts amount).

Crate should have the following structure then:

```
$ls /src

os/
sys/
lib.rs
stats.rs
```

Where:

 * `os/` contains OS-specific traits and implementations, [same as Rust does](https://doc.rust-lang.org/std/os/index.html)
 * `sys/` contains platform-specific implementations
 * `lib.rs` should expose all public types and functions
 * `stats.rs` contains `CpuStats` struct and function which returns `impl Future<Item=CpuStats>`

## Public interface

```rust
use crate::sys;

/// System CPU stats
#[derive(heim_derive::ImplWrap)]
pub struct CpuStats(sys::CpuStats);
```

`heim_derive::ImplWrap` is a `proc-macro` which generates `AsRef`, `AsMut` and `From` implementations,
which allows to work with the "inner" `sys::CpuStats` struct.

Default implementation for struct **MUST** have only these methods, which can be
considered as a cross-platform.

```rust
impl CpuStats {
    pub fn ctx_switches(&self) -> u64 {
        self.as_ref().ctx_switches()
    }

    pub fn interrupts(&self) -> u64 {
        self.as_ref().interrupts()
    }
}
```

Linux additionally provides the "soft interrupts" amount value,
but we can't expose it here, because it would not be portable.\
Instead, we should create the `CpuStatsExt` trait at the `os/linux/stats.rs`:

```rust
#[heim_derive::os_ext_for(crate::CpuStats, cfg(target_os = "linux"))]
pub trait CpuStatsExt {
    fn soft_interrupts(&self) -> u64;
}
```

Another `proc-macro` `heim_derive::os_ext_for` will generate implementation
of our `CpuStatsExt` for `crate::CpuStats` and add the feature gate: `#[cfg(target_os = "linux")]`

## Platform-specific

Now we need to create an platform implementation and we will start with a `sys/linux/times.rs` file.

`sys/linux/mod.rs` should be feature-gated too with `#[cfg(target_os = "linux")]`,
because it can be used only for Linux systems. The same thing applies to all platforms implementations.

This `sys/linux/times.rs` file should contain the struct with the same name,
because we are using it above, but it contents will vary from platform to platform.

In case of Linux it will contain a few fields, which will be populated later:

```rust
pub struct CpuStats {
    ctx_switches: u64,
    interrupts: u64,
    soft_interrupts: u64,
}

impl FromStr for CpuStats {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<CpuStats, Self::Err> {
        // ..
    }
}
```

Now, we need to provide the async interface. In case of Linux we need to parse `/proc/stat` file
and create `sys::CpuStats` struct with data from it.

Our `sys/linux/times.rs` should declare one function:

```rust
pub fn cpu_times() -> impl Future<Item=CpuStats, Error=Error> {
    heim_common::utils::fs::read_into("/proc/stat")
}
```

What will happen here: `/proc/stat` will be read asynchronously with `tokio::fs`
help and it would be parsed with the `FromStr` implementation.

Now let's go back to the public `CpuStats` struct.

It should declare the similar function too, but it's implementation will be much simpler:

```rust
use crate::sys;

pub fn stats() -> impl Future<Item=CpuStats, Error=Error> {
    sys::stats().map(Into::into)
}
```

Since `heim_derive::ImplWrap` macro generates `From<sys::CpuStats for CpuStats` implementation,
all we need now is to call platform-specific function and wrap result into public struct.

Same thing applies to all structs and functions returning `Future`s and `Stream`s --
platform-specific implementation should be received from the `Future` or `Stream` and wrapped
into a public struct via `Into::into`.

## Additional

Please, stick to the [Rust API guidelines](https://rust-lang-nursery.github.io/api-guidelines/checklist.html)
where possible.
