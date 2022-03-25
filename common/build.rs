use quote::quote;

fn is_witx_file(e: &std::fs::DirEntry) -> bool {
    e.path().extension().map(|ext| ext == "witx").unwrap_or(false)
}

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=witx/");

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("expected CARGO_MANIFEST_DIR to be set"));
    let paths: Vec<std::path::PathBuf> = std::fs::read_dir(manifest_dir.join("witx")).expect("Expected `witx` subdirectory to exist")
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(is_witx_file)
                    .map(|e| e.path())
                    .collect();

    for mut path in paths {
        let doc: witx::Document = witx::load(&[path.clone()]).expect("loading witx");

        let names = wiggle_generate::Names::new(quote!(wiggle));

        let settings = wiggle_generate::CodegenSettings::new(
            &wiggle_generate::config::ErrorConf::default(),
            &wiggle_generate::config::AsyncConf::default(),
            &doc,
            false,
        )
        .expect("validating codegen settings");

        let code = wiggle_generate::generate(&doc, &names, &settings);

        let contents = quote!(#code).into_iter().map(|tt| format!("{}", tt)).collect::<Vec<String>>();
        path.set_extension("rs");
        let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR must be set")).join(path.file_name().expect("There's gotta be a filename"));
        //std::fs::write(manifest_dir.join("common-build-log.txt"), diags).expect("Failed to write diagnostic file");
        std::fs::write(out_path, &contents.join("\n")).expect("Failed to write output file");
    }
}