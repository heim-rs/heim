#![feature(async_await)]

use heim_virt::detect;

#[runtime::main]
async fn main() {
    dbg!(detect().await);
}
