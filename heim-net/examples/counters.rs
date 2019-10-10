use heim_common::prelude::*;
use heim_net as net;

#[heim_derive::main]
async fn main() -> Result2<()> {
    let mut counters = net::io_counters();
    while let Some(counter) = counters.next().await {
        dbg!(counter?);
    }

    Ok(())
}
