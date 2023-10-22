use crate::commands::new_c::fill_project_defaults;
use utils::printer::{err_print, info_print, success_print};

pub fn init() {
    // Get the current directory path.
    let current_directory_path = std::env::current_dir()
        .unwrap_or_else(|_| {
            err_print("Unable to get current directory");
            std::process::exit(1);
        })
        .to_str()
        .unwrap()
        .to_string();

    // Extract the directory name from the path.
    let dir_name = String::from(current_directory_path)
        .split("/")
        .last()
        .unwrap()
        .to_string();

    // Print the project initialization message.
    info_print(format!("Initializing project `{}`", &dir_name));

    // Start measuring the time it takes to initialize the project.
    let time = std::time::Instant::now();

    // Fill project defaults.
    fill_project_defaults(".", &dir_name);

    // Print the success message with the elapsed time.
    success_print(&format!(
        "Project successfully initialized in {:?}",
        time.elapsed()
    ));
}
