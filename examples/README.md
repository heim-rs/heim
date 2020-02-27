# Examples of how to use `heim`

All examples can be executed with

```
cargo run --example $name
```

## Async runtimes

It is preferred in general to use either `tokio` or `async-std` runtime
to drive `heim` routines to execution:

### tokio

Note that all these examples are using [`tokio`](https://tokio.rs),
by enabling `runtime-tokio` feature of `heim`:

```toml
[dependencies]
heim = { version = "*", features = ["runtime-tokio"] }
```

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let memory = heim::memory::memory().await?;
    dbg!(memory);

    Ok(())
}
```

### async-std

You can also use [`async-std`](https://async.rs) runtime instead
by using `runtime-async-std` feature:

```toml
[dependencies]
heim = { version = "*", features = ["memory", "runtime-async-std"] }
```

```rust
#[async_std::main]
async fn main() -> Result<()> {
    let memory = heim::memory::memory().await?;
    dbg!(memory);

    Ok(())
}
```

## Fallback option

In case you do not want to use none of these async runtimes
(ex. you are using other async runtime or just to use it in a blocking way),
`heim` provides polyfill option, which executes all internally used,
potentially blocking operations on the current thread.

```toml
[dependencies]
heim = { version = "*", features = ["memory", "runtime-polyfill"] }
futures = "*"
```

```rust
fn main() -> Result<()> {
    let memory = futures::executor::block_on(heim::memory::memory())?;
    dbg!(memory);

    Ok(())
}
```
