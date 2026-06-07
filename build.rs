extern crate prost_build;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=ORTOOLS_PREFIX");
    println!("cargo:rerun-if-changed=src/cp_sat_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/cp_model.proto");
    println!("cargo:rerun-if-changed=src/sat_parameters.proto");

    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    prost_build::compile_protos(
        &["src/cp_model.proto", "src/sat_parameters.proto"],
        &["src/"],
    )
    .unwrap();

    let ortools_prefix = std::env::var("ORTOOLS_PREFIX")
        .ok()
        .unwrap_or_else(|| "/opt/ortools".into());

    let prefix = PathBuf::from(&ortools_prefix);

    cc::Build::new()
        .cpp(true)
        .flags(["-std=c++17", "-DOR_PROTO_DLL="])
        .file("src/cp_sat_wrapper.cpp")
        .include(prefix.join("include"))
        .compile("cp_sat_wrapper.a");

    println!("cargo:rustc-link-lib=ortools");
    println!("cargo:rustc-link-lib=protobuf");

    let lib = prefix.join("lib");
    let lib64 = prefix.join("lib64");

    for dir in [&lib, &lib64] {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "linux" {
        for dir in [&lib, &lib64] {
            if dir.exists() {
                println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dir.display());
            }
        }
    }
}
