use std::{env, fs, path::PathBuf, process::Command};

const VENDOR_DIR: &str = "./vendor";
const LIBRARY_NAME: &str = "libnewrelic.a";

fn main() {
    let vendor_path = PathBuf::from(VENDOR_DIR);
    let out_dir = env::var("OUT_DIR").unwrap();
    // Tell cargo to tell rustc where to find the nr_agent_sdk
    // shared library.
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=newrelic");
    println!("cargo:rustc-link-lib=dylib=pcre");

    Command::new("make")
        .arg("static")
        .current_dir(vendor_path.clone())
        .status()
        .expect("Running 'make' failed. Ensure your environment meets the minimum requirements to build the New Relic C SDK: https://github.com/newrelic/c-sdk#requirements");

    // Copy the object files into the $OUT_DIR directory to be linked against.
    let mut obj = vendor_path.clone();
    obj.push(LIBRARY_NAME);
    println!("{:?}", obj);
    fs::copy(&obj, format!("{}/{}", out_dir, LIBRARY_NAME)).expect("Could not copy object files");

    Command::new("make")
        .arg("clean")
        .current_dir(vendor_path.clone())
        .status()
        .expect("Could not clean up package directory");
}
