use std::env;

fn main() {
    use std::path::Path;

    let crate_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Missing 'CARGO_MANIFEST_DIR' environment variable.");

    let crate_dir = Path::new(&crate_dir);

    let config_path = crate_dir.join("cbindgen.toml");
    let header_path = crate_dir.join("include/rust_async_executor.h");

    let config = cbindgen::Config::from_file(config_path)
        .expect("Unable to read cbindgen config");

    std::fs::create_dir_all(header_path.parent().unwrap())
        .expect("Unable to create header directory");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}
