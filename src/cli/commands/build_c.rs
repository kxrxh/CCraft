use crate::{
    config::{
        loader::{load_config, validate},
        types::Config,
    },
    utils::{
        cmd::check_command_exists,
        file::search_files_with_ext,
        folder::{create_folder, is_folder_exists},
    },
};
use std::process::Command;

const EXTENSIONS: [&str; 1] = ["c"];

pub(crate) fn build_project() {
    run_build(load_config());
}

fn run_build(config: Config) {
    validate(&config);
    let compiler = config.get_build().get_compiler();
    let is_compiler_exists = check_command_exists(compiler);
    if !is_compiler_exists {
        eprintln!("❌ Unable to find compiler {}", compiler);
        std::process::exit(2);
    }
    let mut build_command = Command::new(config.get_build().get_compiler());
    // creating the build folder
    if let Err(_) = create_folder(".", "build") {
        if !is_folder_exists("build") {
            eprintln!("❌ Unable to create build folder");
            std::process::exit(1);
        }
    }
    // Search for C files
    let files = search_files_with_ext(&EXTENSIONS, "src");
    build_command.args(files);

    // Setting up the output directory
    let output_path = std::path::PathBuf::from("build").join(config.get_project().get_name());
    build_command.arg("-o").arg(output_path);

    // Setting up user's flags
    for flag in config.get_build().get_flags() {
        build_command.arg(flag);
    }

    println!("🏗️ Building project...");
    let time = std::time::Instant::now();
    let status = build_command.status().unwrap_or_else(|_| {
        eprintln!("❌ Failed to run build command");
        std::process::exit(1);
    });

    if !status.success() {
        eprintln!(
            "❌ Build failed with exit code {}",
            status.code().unwrap_or(-1)
        );
        std::process::exit(2);
    }

    println!("✅ Build completed in {}ms", time.elapsed().as_millis());
}
