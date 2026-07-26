fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let slint_dir = std::path::Path::new(&manifest_dir).join("slint");

    let mut config = slint_build::CompilerConfiguration::new();
    config = config.with_include_paths([slint_dir.clone()].to_vec());

    slint_build::compile_with_config(slint_dir.join("ltk.slint"), config).unwrap();
}
