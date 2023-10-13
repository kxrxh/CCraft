use crate::utils::{file::is_file_exist, printer::err_print};

use super::types::Config;

pub(crate) fn load_config() -> Config {
    if !is_file_exist(".", "config.toml") {
        err_print("Config file not found.");
        std::process::exit(1);
    }
    let file = std::fs::read_to_string("config.toml").unwrap_or_else(|_| {
        err_print("Unable to read config file!");
        std::process::exit(1);
    });
    Config::deserialize(&file)
}

/// Validates the given configuration.
/// Exits the process with an error code if any validation fails.
pub(crate) fn validate(config: &Config) {
    // Check if the project name is empty
    match config.get_project().get_name().is_empty() {
        true => {
            err_print("Project name is empty!");
            std::process::exit(1);
        }
        _ => (),
    }

    // Check if the compiler is empty
    match config.get_build().get_compiler().is_empty() {
        true => {
            err_print("Compiler is not set!");
            std::process::exit(1);
        }
        _ => (),
    }

    // Check if the source directory is empty
    match config.get_build().get_source_dir().is_empty() {
        true => {
            err_print("Source directory is empty!");
            std::process::exit(1);
        }
        _ => (),
    }

    // Check if the output directory is empty
    match config.get_build().get_output_dir().is_empty() {
        true => {
            err_print("Output directory is empty!");
            std::process::exit(1);
        }
        _ => (),
    }
}
