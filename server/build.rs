use std::env;

fn main() {
    let v = format!("v{} ", env!("CARGO_PKG_VERSION"));

    println!("cargo::rustc-env=AEGIS_VERSION={v}");
}
