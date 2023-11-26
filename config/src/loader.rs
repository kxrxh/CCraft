use utils::{file::is_file_exist, printer::err_print};

use crate::types::Config;

/// Loads the configuration from the "craft.toml" file.
///
/// # Panics
///
/// This function will panic if the config file is not found or unable to be read.
pub fn load_config() -> Config {
    // Check if the config file exists
    if !is_file_exist(".", "craft.toml") {
        err_print("Config file not found.");
        std::process::exit(1);
    }

    // Read the config file
    let file = std::fs::read_to_string("craft.toml").unwrap_or_else(|_| {
        err_print("Unable to read config file!");
        std::process::exit(1);
    });

    // Deserialize the config file contents into a Config struct
    Config::deserialize(&file)
}

/// Validates the given configuration.
///
/// # Arguments
///
/// * `config` - The configuration to validate.
pub fn validate(config: &Config) {
    // Check if the project name is empty
    if config.get_project().get_name().is_empty() {
        err_print("Project name is empty!");
        std::process::exit(1);
    }

    // Check if the compiler is empty
    if config.get_build().get_compiler().is_empty() {
            err_print("Compiler is not set!");
            std::process::exit(1);
    }

    // Check if the source directory is empty
    if config.get_build().get_source_dir().is_empty() {
            err_print("Source directory is empty!");
            std::process::exit(1);
    }

    // Check if the output directory is empty
    if config.get_build().get_output_dir().is_empty() {
            err_print("Output directory is empty!");
            std::process::exit(1);    
    }
}
