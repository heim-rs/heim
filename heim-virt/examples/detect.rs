use heim_virt::detect;

#[heim_derive::main]
async fn main() {
    dbg!(detect().await);
}
