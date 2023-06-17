fn main() {
    #[cfg(feature = "uv")]
    {
        let pkgconfig = pkg_config::Config::new();

        if pkgconfig.probe("libuv").is_err() {
            println!("cargo:rustc-link-lib=uv");
        }
    }

    #[cfg(feature = "cbindgen")]
    {
        use std::{env::var, path::Path};

        let profile = var("PROFILE")
            .expect("Unfortunately missing 'PROFILE' environment variable.");

        let out_dir = var("OUT_DIR")
            .expect("Unfortunately missing 'OUT_DIR' environment variable.");
        let out_dir = Path::new(&out_dir);

        let target_dir = out_dir.ancestors().find(|path| match path.file_name() {
            Some(name) if std::ffi::OsStr::new(&profile) == name => true,
            _ => false,
        }).expect("Unable to determine target directory");

        let source_dir = var("CARGO_MANIFEST_DIR")
            .expect("Unfortunately missing 'CARGO_MANIFEST_DIR' environment variable.");
        let source_dir = Path::new(&source_dir);

        let config_path = source_dir.join("cbindgen.toml");
        let config = cbindgen::Config::from_file(config_path)
            .expect("Unable to read cbindgen config");

        let header_dir = target_dir.join("include");

        std::fs::create_dir_all(&header_dir)
            .expect("Unable to create headers directory");

        for (needed, header_file, sym_filter) in &[
            (true, "rust_async_executor.h", (|name| !name.contains("_dart_") && !name.contains("_uv_")) as fn(&str) -> bool),
            (cfg!(feature = "uv"), "rust_async_executor_uv.h", |name| name.contains("_uv_")),
            (cfg!(feature = "dart"), "rust_async_executor_dart.h", |name| name.contains("_dart_"))
        ] {
            if *needed {
                let header_path = header_dir.join(&header_file);
                let header_guard = format!("__{}__", header_file.replace('.', "_").to_uppercase());
                let mut config = config.clone();

                config.include_guard = header_guard.into();
                config.export.exclude.extend(config.export.include.iter().filter(|name| !sym_filter(&name)).cloned());
                config.export.include = Vec::default();

                cbindgen::Builder::new()
                    .with_crate(source_dir)
                    .with_config(config)
                    .generate()
                    .expect("Unable to generate bindings")
                    .write_to_file(header_path);
            }
        }

        println!("cargo:rerun-if-changed=cbindgen.toml");
    }
}
