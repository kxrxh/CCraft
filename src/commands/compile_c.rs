use std::fs;

use config::{
    loader::{load_config, validate},
    types::Config,
};
use utils::{
    directory::{create_directory, is_directory_exists},
    file::{get_modification_time, search_files_with_ext},
    other::check_command_exists,
    printer::{err_print, info_print, success_print},
};

use crate::build_system::{
    building::compile_file,
    cache::{BuildCache, BuildTimeFileCache},
};

pub(crate) fn compile_project() {
    let config = load_config();
    validate(&config);
    start_compiling(&config);
}

/// Start compiling the project using the provided configuration.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` object containing the build configuration.
pub(crate) fn start_compiling(config: &Config) {
    let compiler = config.get_build().get_compiler();

    if !check_command_exists(compiler) {
        err_print(&format!("Unable to find compiler `{}`", compiler));
        std::process::exit(2);
    }

    // creating the build directory
    create_directory(".", "build").unwrap_or_else(|_| {
        if !is_directory_exists("build") {
            err_print("Unable to create build dir");
            std::process::exit(1);
        }
    });

    // loading build cache.
    let mut build_cache = load_build_cache();

    // creating object directory
    create_directory("build", "obj").unwrap_or_else(|_| {
        if !is_directory_exists("obj") {
            err_print("Unable to create objects dir");
            std::process::exit(1);
        }
    });

    // setting up compilation start time
    let time = std::time::Instant::now();
    info_print(&format!("Compiling project using `{compiler}`..."));

    // Search for C files
    // ? Still works in single thread way. May be improved in the future.

    let files = search_files_with_ext(&["c"], "src");
    for (index, file) in files.iter().enumerate() {
        let cache_key = file.clone();

        if let Some(cache_entry) = build_cache
            .files_cache
            .iter_mut()
            .find(|entry| entry.path_eq(&cache_key))
        {
            if cache_entry.get_time() >= get_modification_time(file).unwrap() {
                info_print(&format!(
                    "[{}/{}] Skipping file: {} (no changes)",
                    index + 1,
                    files.len(),
                    file
                ));
                continue;
            }

            info_print(&format!(
                "[{}/{}] Compiling file: {}",
                index + 1,
                files.len(),
                file
            ));
            compile_file(compiler, &file, config.get_build().get_compile_flags());
            cache_entry.update_time(get_modification_time(file).unwrap());
            continue;
        } else {
            info_print(&format!(
                "[{}/{}] Compiling file: {}",
                index + 1,
                files.len(),
                file
            ));
            compile_file(compiler, &file, config.get_build().get_compile_flags());
            build_cache
                .files_cache
                .push(BuildTimeFileCache::new(file, vec![]));
        }

        info_print(&format!(
            "[{}/{}] Compiling file: {}",
            index + 1,
            files.len(),
            file
        ));
        compile_file(compiler, &file, config.get_build().get_compile_flags());
    }

    save_build_cache(&build_cache);

    success_print(&format!("Compiling completed in {:?}", time.elapsed()));
}

fn load_build_cache() -> BuildCache {
    if let Ok(contents) = fs::read_to_string("build/cache.json") {
        if let Ok(cache) = BuildCache::new_from_json(&contents) {
            return cache;
        }
    }
    BuildCache {
        files_cache: Vec::new(),
    }
}

// Function to save build cache to file
fn save_build_cache(cache: &BuildCache) {
    if let Ok(json) = cache.to_json_string() {
        if let Err(err) = fs::write("build/cache.json", json) {
            eprintln!("Error saving build cache: {}", err);
        }
    }
}
