use crate::{
    config::{
        loader::{load_config, validate},
        types::Config,
    },
    utils::{
        cmd::check_command_exists,
        file::search_files_with_ext,
        folder::{create_folder, is_folder_exists},
        printer::{err_print, info_print, success_print, warn_print},
    },
};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

const EXTENSIONS: [&str; 1] = ["c"];

pub(crate) fn build_project() {
    run_build(load_config());
}

fn run_build(config: Config) {
    validate(&config);
    let compiler = config.get_build().get_compiler();
    let is_compiler_exists = check_command_exists(compiler);
    if !is_compiler_exists {
        err_print(format!("Unable to find compiler `{}`", compiler));
        std::process::exit(2);
    }
    let mut build_command = Command::new(config.get_build().get_compiler());
    // creating the build folder
    if let Err(_) = create_folder(".", "build") {
        if !is_folder_exists("build") {
            err_print("Unable to create build folder");
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

    info_print(format!("Building project using `{compiler}`..."));

    let mut child = build_command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = child.stdout {
        for line in BufReader::new(stdout).lines() {
            let line = line.unwrap();
            info_print(line);
        }
    }

    if let Some(ref mut stderr) = child.stderr {
        let mut is_err = false; // is value is false then print as warn
        let lines: Vec<Result<String, _>> = BufReader::new(stderr).lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let line = line.as_ref().unwrap();
            if index == lines.len() - 1 {
                info_print(line);
                continue;
            }
            if line.contains(": error: ") {
                err_print(line);
                is_err = true;
            } else {
                if is_err {
                    err_print(line);
                    continue;
                }
                if line.contains(": warning: ") {
                    is_err = false;
                }
                warn_print(line)
            }
        }
    }
    let time = std::time::Instant::now();
    let status = child.wait().unwrap_or_else(|_| {
        err_print("Failed to run build command");
        std::process::exit(1);
    });

    if status.success() {
        success_print(format!("Build completed in {:?}", time.elapsed()));
        return;
    }
    
    err_print(format!(
        "Build failed with exit code {}",
        status.code().unwrap_or(-1)
    ));
    std::process::exit(2);
}
