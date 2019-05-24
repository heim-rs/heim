#![feature(async_await)]

use heim_common::prelude::*;
use heim_net as net;

#[runtime::main]
async fn main() -> Result<()> {
    let mut nic = net::nic();
    while let Some(iface) = nic.next().await {
        dbg!(iface?);
    }

    Ok(())
}
