fn main() {
    //#[cfg(feature = "bindgen")]
    {
        use std::{env::var, path::Path};

        let package = var("CARGO_PKG_NAME")
            .expect("Unfortunately missing 'CARGO_PKG_NAME' environment variable.");

        let profile =
            var("PROFILE").expect("Unfortunately missing 'PROFILE' environment variable.");

        let out_dir =
            var("OUT_DIR").expect("Unfortunately missing 'OUT_DIR' environment variable.");
        let out_dir = Path::new(&out_dir);

        let target_dir = out_dir
            .ancestors()
            .find(|path| matches!(path.file_name(), Some(name) if std::ffi::OsStr::new(&profile) == name))
            .expect("Unable to determine target directory");

        let source_dir = var("CARGO_MANIFEST_DIR")
            .expect("Unfortunately missing 'CARGO_MANIFEST_DIR' environment variable.");
        let source_dir = Path::new(&source_dir);

        let header_file = format!("rust_{}.h", package);

        let config_path = source_dir.join("cbindgen.toml");
        let header_path = target_dir.join("include").join(header_file);

        let config =
            cbindgen::Config::from_file(config_path).expect("Unable to read cbindgen config");

        std::fs::create_dir_all(header_path.parent().unwrap())
            .expect("Unable to create header directory");

        cbindgen::Builder::new()
            .with_crate(source_dir)
            .with_config(config)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(header_path);
    }
}
