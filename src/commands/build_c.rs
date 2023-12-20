use std::process::Command;

use config::{
    loader::{load_config, validate},
    types::Config,
};
use utils::{
    file::search_files_with_ext,
    other::execute_command,
    printer::{err_print, success_print},
};

use super::compile_c::start_compiling;

pub(crate) fn build_project() {
    let config = load_config();
    validate(&config);
    start_compiling(&config);
    start_linking(&config);
}

/// Starts the linking process.
///
/// # Arguments
///
/// * `config` - The configuration object.
pub(crate) fn start_linking(config: &Config) {
    // Search for object files with the extension "o" in the "build/obj" directory
    let files: Vec<String> = search_files_with_ext(&["o"], "build/obj");

    // Get the compiler from the build configuration
    let compiler = config.get_build().get_compiler();

    // Create the output file path
    let output_file_path =
        std::path::PathBuf::from("build/".to_string() + config.get_project().get_name());

    // Create the command to execute the compiler
    let mut binding = Command::new(compiler);
    let mut command = binding.arg("-o").arg(output_file_path).args(files);

    // Execute the command
    let result = execute_command(&mut command);

    // Handle errors
    if result.is_err() {
        err_print("Unable to link files");
        std::process::exit(1);
    }

    // Print success message
    success_print("Linking completed.");
}
