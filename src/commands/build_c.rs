use config::loader::{load_config, validate};




pub(crate) fn build_project() {
    let config = load_config();
    validate(&config);
}

// fn run_build(config: Config) {
//     validate(&config);
//     let compiler = config.get_build().get_compiler();
//     let is_compiler_exists = check_command_exists(compiler);
//     if !is_compiler_exists {
//         err_print(format!("Unable to find compiler `{}`", compiler));
//         std::process::exit(2);
//     }
//     let mut build_command = Command::new(config.get_build().get_compiler());
//     // creating the build directory
//     if let Err(_) = create_directory(".", "build") {
//         if !is_directory_exists("build") {
//             err_print("Unable to create build directory");
//             std::process::exit(1);
//         }
//     }
//     // Search for C files
//     let files = search_files_with_ext(&EXTENSIONS, "src");
//     build_command.args(files);

//     // Setting up the output directory
//     let output_path = std::path::PathBuf::from("build").join(config.get_project().get_name());
//     build_command.arg("-o").arg(output_path);

//     // Setting up user's flags
//     for flag in config.get_build().get_build_flags() {
//         build_command.arg(flag);
//     }

//     info_print(format!("Building project using `{compiler}`..."));

//     let mut child = build_command
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()
//         .unwrap();

//     if let Some(ref mut stdout) = child.stdout {
//         for line in BufReader::new(stdout).lines() {
//             let line = line.unwrap();
//             info_print(line);
//         }
//     }

//     let time = std::time::Instant::now();
//     let status = child.wait().unwrap_or_else(|_| {
//         err_print("Failed to run build command");
//         std::process::exit(1);
//     });

//     if status.success() {
//         success_print(format!("Build completed in {:?}", time.elapsed()));
//         return;
//     }

//     err_print(format!(
//         "Build failed with exit code {}",
//         status.code().unwrap_or(-1)
//     ));
//     std::process::exit(2);
// }

