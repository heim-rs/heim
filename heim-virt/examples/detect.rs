#![allow(stable_features)]
#![feature(async_await, futures_api)]

use heim_virt::detect;

#[runtime::main]
async fn main() {
    dbg!(detect().await);
}
