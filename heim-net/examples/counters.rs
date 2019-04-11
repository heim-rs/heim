use heim_common::prelude::*;
use heim_net as net;
use heim_runtime::{self as runtime, SyncRuntime};

fn main() -> Result<()> {
    let mut rt = runtime::new().unwrap();
    for io_cnt in rt.block_collect(net::io_counters()) {
        println!("{:?}", io_cnt);
    }

    Ok(())
}
