use std::env;

fn main() {
    let package_name = "postal";
    let shared_libs = ["postal"];

    if let Ok(inc_dir) = env::var("POSTAL_INCLUDE_DIR") {
        println!("cargo:include={}", inc_dir);
    }
    if let Ok(lib_dir) = env::var("POSTAL_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return;
    }
    println!("cargo:rustc-link-lib=dylib={}", package_name);
}
