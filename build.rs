use std::env;
use std::path::{PathBuf};

use bindgen::Builder;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let epics_base = PathBuf::from(env::var("EPICS_BASE").unwrap());
    let bindings = Builder::default()
        .clang_arg(format!("-I{}", epics_base.join("include").to_str().unwrap()))
        .clang_arg(format!("-I{}", epics_base.join("include/os/Linux").to_str().unwrap()))
        .clang_arg(format!("-I{}", epics_base.join("include/compiler/clang").to_str().unwrap()))
        .header(epics_base.join("include/callback.h").to_str().unwrap())
        .header(epics_base.join("include/dbScan.h").to_str().unwrap())
        .header(epics_base.join("include/dbCommon.h").to_str().unwrap())
        .header(epics_base.join("include/aiRecord.h").to_str().unwrap())
        .header(epics_base.join("include/aoRecord.h").to_str().unwrap())
        .header(epics_base.join("include/biRecord.h").to_str().unwrap())
        .header(epics_base.join("include/boRecord.h").to_str().unwrap())
        .header(epics_base.join("include/longinRecord.h").to_str().unwrap())
        .header(epics_base.join("include/longoutRecord.h").to_str().unwrap())
        .header(epics_base.join("include/stringinRecord.h").to_str().unwrap())
        .header(epics_base.join("include/stringoutRecord.h").to_str().unwrap())
        .header(epics_base.join("include/devSup.h").to_str().unwrap())
        .header(epics_base.join("include/iocsh.h").to_str().unwrap())
        .derive_default(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("epics.rs"))
        .expect("Couldn't write bindings!");
}
