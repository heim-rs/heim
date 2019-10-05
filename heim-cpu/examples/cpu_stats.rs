use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::main]
async fn main() -> Result2<()> {
    dbg!(cpu::stats().await?);

    Ok(())
}
