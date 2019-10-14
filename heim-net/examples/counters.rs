use heim_common::prelude::*;
use heim_net as net;

#[heim_derive::main]
async fn main() -> Result<()> {
    let mut counters = net::io_counters().boxed();
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    Ok(())
}
