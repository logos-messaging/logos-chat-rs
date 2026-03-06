fn main() -> Result<(), std::io::Error> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let libs_dir = std::path::Path::new(&manifest_dir).join("libs");
    println!("cargo:rustc-link-search=native={}", libs_dir.display());

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "macos" | "linux" => {
            println!("cargo:rustc-link-lib=dylib=waku");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
        }
        other => {
            panic!("Unsupported target OS: {other}. Only macOS and Linux are supported.");
        }
    }

    Ok(())
}
