#![feature(async_await)]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[runtime::main]
async fn main() -> Result<()> {
    dbg!(cpu::stats().await?);

    Ok(())
}
