use std::env;

fn main() {
    println!("cargo:rustc-link-lib=ralloc");
    println!(
        "cargo:rustc-link-search={}",
        env::var("LIB_RALLOC_BUILD_DIR").unwrap()
    );
}
