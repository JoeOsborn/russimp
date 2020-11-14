use std::{env::var, path::PathBuf};

const BINDINGS_FILE : &str = "bindings.rs";
const WRAPPER_FILE : &str = "wrapper.h";

fn main() {
    let assimp_path = cmake::Config::new("assimp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    let path_buf_src = PathBuf::from(var("OUT_DIR").unwrap()).join(format!("../../../../../russimp-sys/src/{}", BINDINGS_FILE));
    let path_file_src = path_buf_src.as_os_str().to_str().unwrap();

    bindgen::Builder::default()
        .header(WRAPPER_FILE)
        .clang_args(&["-I", assimp_path.join("include").to_str().unwrap()])
        .whitelist_function("aiImportFile")
        .whitelist_type("aiPostProcessSteps")
        .whitelist_type("aiPrimitiveType")
        .whitelist_function("aiReleaseImport")
        .whitelist_function("aiGetErrorString")
        .generate()
        .unwrap()
        .write_to_file(path_file_src)
        .unwrap();
}