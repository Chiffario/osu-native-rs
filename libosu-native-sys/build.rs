use std::{env, error::Error, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    let outdir = env::var("CARGO_MANIFEST_DIR").or(Err("No output directory"))?;
    println!("cargo:rustc-link-search={}", outdir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", outdir);
    Ok(())
}
