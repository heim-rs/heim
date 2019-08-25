use heim_virt as virt;

#[heim_derive::test]
async fn smoke_detect() {
    let _ = virt::detect().await;
}
