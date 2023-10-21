use std::process::Command;

use config::{
    loader::{load_config, validate},
    types::Config,
};
use utils::{
    directory::{create_directory, is_directory_exists},
    file::search_files_with_ext,
    other::{check_command_exists, execute_command},
    printer::{err_print, info_print, success_print},
};

pub(crate) fn compile_project() {
    let config = load_config();
    validate(&config);
    start_compiling(&config);
}

pub(crate) fn start_compiling(config: &Config) {
    let compiler = config.get_build().get_compiler();
    let is_compiler_exists = check_command_exists(compiler);
    if !is_compiler_exists {
        err_print(format!("Unable to find compiler `{}`", compiler));
        std::process::exit(2);
    }
    // creating the build directory
    if let Err(_) = create_directory(".", "build") {
        if !is_directory_exists("build") {
            err_print("Unable to create build dir");
            std::process::exit(1);
        }
    }
    // creating object directory
    if let Err(_) = create_directory("build", "obj") {
        if !is_directory_exists("obj") {
            err_print("Unable to create objects dir");
            std::process::exit(1);
        }
    }

    let time = std::time::Instant::now();
    info_print(format!("Compiling project using `{compiler}`..."));

    // Search for C files
    // ? Still works in single thread way. May be improved in the future.

    let files = search_files_with_ext(&["c"], "src");
    for (index, file) in files.iter().enumerate() {
        info_print(format!(
            "[{}/{}] Compiling file: {}",
            index + 1,
            files.len(),
            file
        ));
        compile_file(compiler, &file, config.get_build().get_compile_flags());
    }

    success_print(format!("Compiling completed in {:?}", time.elapsed()));
}

fn compile_file(compiler: &str, file: &str, flags: &Vec<String>) {
    let filename = file.split("/").last().unwrap();
    // Retrieve file name from path and replace .c with .o
    let output_file_name = filename.replace(".c", ".o");

    let output_path = std::path::PathBuf::from("build/obj").join(output_file_name);

    let mut binding = Command::new(compiler);
    let mut command = binding
        .arg("-c")
        .arg(file)
        .arg("-o")
        .arg(output_path)
        .args(flags);

    let result = execute_command(&mut command);
    if result.is_err() {
        err_print("Unable to compile file: ".to_string() + filename);
        std::process::exit(1);
    }
}
