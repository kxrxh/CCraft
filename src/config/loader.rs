use crate::utils::file::is_file_exist;

use super::types::Config;

pub(crate) fn load_config() -> Option<Config> {
    if !is_file_exist(".", "config.toml") {
        eprintln!("Config file not found.");
        return None;
    }
    let file = std::fs::read_to_string("config.toml");
    if file.is_err() {
        eprintln!("Unable to read config file!");
        return None;
    }
    Some(Config::deserialize(&file.unwrap()))
}

pub(crate) fn fill_with_default(config: &mut Config) {}

pub(crate) fn validate(config: &Config) {
    if config.get_project().get_name().is_empty() {
        eprintln!("Project name is empty!");
        std::process::exit(1);
    }

    if config.get_build().get_compiler().is_empty() {
        eprintln!("Compiler is empty!");
        std::process::exit(1);
    }

    if config.get_build().get_source_dir().is_empty() {
        eprintln!("Source directory is empty!");
        std::process::exit(1);
    }

    if config.get_build().get_output_dir().is_empty() {
        eprintln!("Output directory is empty!");
        std::process::exit(1);
    }

    if config.get_target().get_name().is_empty() {
        eprintln!("Target name is empty!");
        std::process::exit(1);
    }
}
