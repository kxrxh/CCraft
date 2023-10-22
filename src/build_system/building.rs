use std::process::Command;

use utils::{other::execute_command, printer::err_print};

/// Compiles a file using the specified compiler, file path, and flags.
///
/// # Arguments
///
/// * `compiler` - The compiler to use for compilation.
/// * `file` - The path of the file to compile.
/// * `flags` - Additional flags to pass to the compiler.
///
pub fn compile_file(compiler: &str, file: &str, flags: &Vec<String>) {
    // Retrieve the file name from the path
    let filename = file.split("/").last().unwrap();

    // Replace ".c" with ".o" in the filename
    let output_file_name = filename.replace(".c", ".o");

    // Create the output path for the compiled file
    let output_path = std::path::PathBuf::from("build/obj").join(output_file_name);

    // Create the command for executing the compiler
    let mut binding = Command::new(compiler);
    let mut command = binding
        .arg("-c")
        .arg(file)
        .arg("-o")
        .arg(output_path)
        .args(flags);

    // Execute the command and check for errors
    let result = execute_command(&mut command);
    if result.is_err() {
        err_print("Unable to compile file: ".to_string() + filename);
        std::process::exit(1);
    }
}
