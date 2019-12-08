use std::env;

fn main() {
    if let Ok("macos") = env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        println!("cargo:rustc-link-lib=framework=IOKit");
    }
}
