#![allow(stable_features)]
#![feature(await_macro, async_await, futures_api)]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[cfg(target_os = "linux")]
async fn linux_frequencies() -> Result<()> {
    let mut frequencies = cpu::os::linux::frequencies();
    while let Some(freq) = await!(frequencies.next()) {
        dbg!(freq?);
    }

    Ok(())
}

#[runtime::main]
async fn main() -> Result<()> {
    let freq = await!(cpu::frequency());
    dbg!(freq?);

    #[cfg(target_os = "linux")]
    await!(linux_frequencies())?;

    Ok(())
}
