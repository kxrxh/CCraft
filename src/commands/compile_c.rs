use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use config::{
    loader::{load_config, validate},
    types::Config,
};
use utils::{
    directory::{create_directory, is_directory_exists},
    file::search_files_with_ext,
    other::check_command_exists,
    printer::{err_print, info_print, success_print, warn_print},
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
    // ? Single thread way. May be improved in the future.

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

enum LineType {
    Error,
    Warning,
    Other,
}

fn compile_file(compiler: &str, file: &str, flags: &Vec<String>) {
    // Retrieve file name from path and replace .c with .o
    let output_file_name = file.split("/").last().unwrap().replace(".c", ".o");

    let output_path = std::path::PathBuf::from("build/obj").join(output_file_name);

    let mut binding = Command::new(compiler);
    let command = binding
        .arg("-c")
        .arg(file)
        .arg("-o")
        .arg(output_path)
        .args(flags);

    // * Debug print
    // * println!("Compile command: {:?}", &command);

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Printing stderr
    if let Some(ref mut stderr) = child.stderr {
        let mut current_type = LineType::Other;

        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();

            // Check if the line contains an error
            if line.contains(": error: ") {
                current_type = LineType::Error;
            } else if line.contains(": warning: ") {
                current_type = LineType::Warning;
            }

            match current_type {
                LineType::Error => err_print(line),
                LineType::Warning => warn_print(line),
                LineType::Other => info_print(line),
            }
        }
    }

    // Printing stdout
    if let Some(ref mut stdout) = child.stdout {
        for line in BufReader::new(stdout).lines() {
            let line = line.unwrap();
            info_print(line);
        }
    }

    if child.wait().is_err() {
        err_print("Failed to run compile command");
        std::process::exit(1);
    };
}
