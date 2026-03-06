fn main() {
    // libwaku.dylib lives in <workspace_root>/libs/
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let libs_dir = std::path::Path::new(&manifest_dir)
        .parent()
        .expect("chat package should have a parent directory")
        .join("libs");

    println!("cargo:rustc-link-search=native={}", libs_dir.display());
    println!("cargo:rustc-link-lib=dylib=waku");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
}
