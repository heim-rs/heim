use heim_common::prelude::*;
use heim_cpu as cpu;

#[heim_derive::main]
async fn main() -> Result<()> {
    dbg!(cpu::time().await?);

    let mut times = cpu::times();
    while let Some(time) = times.next().await {
        dbg!(time?);
    }

    Ok(())
}
