use std::env::var;

fn main() {
    let target = var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    if target.contains("windows") {
        println!("cargo:rerun-if-env-changed=WINAPI_NO_BUNDLED_LIBRARIES");
        println!("cargo:rerun-if-env-changed=WINAPI_STATIC_NOBUNDLE");

        // For `src/sys/windows/wtsapi32.rs`
        println!("cargo:rustc-link-lib=wtsapi32");
    }
}
