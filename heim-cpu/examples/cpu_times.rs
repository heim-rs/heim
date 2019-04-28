#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[runtime::main]
async fn main() -> Result<()> {
    dbg!(await!(cpu::time()));

    let mut times = cpu::times();
    while let Some(time) = await!(times.next()) {
        dbg!(time?);
    }

    Ok(())
}
