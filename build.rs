use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    if target == "x86_64-pc-windows-msvc" {
        println!("cargo:rustc-link-search=native={}/../../../../../lib/",out_dir);
        println!("cargo:rustc-link-lib=dylib=msvcrt");
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK64");
    }else if target == "x86_64-unknown-linux-gnu" {
        println!("cargo:rustc-link-search=native={}/../../../../../lib/",out_dir);
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK64");
    } else {
        panic!("Unknown target: {}", target);
    }
}
