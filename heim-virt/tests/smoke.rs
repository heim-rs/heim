#![feature(async_await)]

use heim_virt as virt;

#[runtime::test]
async fn smoke_detect() {
    let _ = virt::detect().await;
}
