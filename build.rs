use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=TG_SYS_STATIC");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-lib=tg");

    let mut cc = cc::Build::new();
    build_tg(&mut cc);
}

fn build_tg(cc: &mut cc::Build) {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lib = dst.join("lib");

    cc.warnings(false).out_dir(&lib).include("deps/tg");

    cc.file("deps/tg/tg.c");

    //if !cfg!(feature="atomics") {
    //    cc.define("TG_NOATOMICS", None);
    //}

    cc.compile("tg");

    let include_dir = dst.join("include");
    fs::create_dir_all(&include_dir).unwrap();
    fs::copy("deps/tg/tg.h", include_dir.join("tg.h")).unwrap();

    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", lib.to_str().unwrap());
    println!("cargo:include={}/include", include_dir.to_str().unwrap());
}
