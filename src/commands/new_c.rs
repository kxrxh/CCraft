use std::io::Write;

use config;
use utils::{
    self,
    directory::{create_directory, is_directory_exists, validate_directory_name},
    file::create_file,
    other::init_git_repository,
    printer::{err_print, info_print, success_print},
};

pub(crate) fn get_project_name_from_user() -> String {
    print!("Enter project name -> ");
    std::io::stdout().flush().unwrap_or_else(|e| {
        err_print(&format!("Failed to flush stdout: {}", e));
        std::process::exit(1);
    });

    let mut user_input = String::new();
    match std::io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let trimmed_input = user_input.trim();
            if trimmed_input.is_empty() {
                err_print("Project name cannot be empty.");
                std::process::exit(1);
            }
            trimmed_input.to_string()
        }
        Err(e) => {
            err_print(&format!("Failed to read user input: {}", e));
            std::process::exit(1);
        }
    }
}

pub(crate) fn new_project_bin(project_name: &str) {
    // Validating project name
    if !validate_directory_name(&project_name) {
        err_print("Invalid directory/project name!");
        info_print(
            "Please use only alphanumeric characters, underscores, and hyphens for project name.",
        );
        std::process::exit(1);
    }

    // Checking if directory exists. If it does, check if it is empty.
    if is_directory_exists(&project_name) {
        err_print("Directory already exists!");
        std::process::exit(1);
    }

    let time = std::time::Instant::now();
    // Creating directory
    info_print(format!("Creating new project `{}`...", &project_name));

    let res = create_directory(".", &project_name);

    if res.is_err() {
        err_print("Unable to create directory!");
        err_print(res.unwrap_err());
        std::process::exit(1);
    }

    // Project directory was created
    // Now creating default project files
    fill_project_defaults(&project_name, &project_name);

    if let Err(err) = init_git_repository(&project_name) {
        err_print("Unable to initialize Git repository!");
        err_print(err);
        std::process::exit(1);
    }

    success_print(&format!(
        "Project successfully created in {:?}",
        time.elapsed()
    ));
}

/// Fill project defaults for a given project name.
///
/// # Arguments
///
/// * `project_name` - The name of the project.
///
pub fn fill_project_defaults(project_directory: &str, project_name: &str) {
    // Creating README.md
    if let Err(err) = create_file(
        project_directory,
        "README.MD",
        &format!("# {}", project_name),
    ) {
        err_print("Unable to create README.MD!");
        err_print(err);
        std::process::exit(1);
    }

    // Creating src directory
    let src_path = std::path::PathBuf::from(project_directory).join("src");
    if let Err(err) = utils::directory::create_directory(project_directory, "src") {
        err_print("Unable to create src directory!");
        err_print(err);
        std::process::exit(1);
    }

    // Creating include directory
    if let Err(err) = utils::directory::create_directory(project_directory, "include") {
        err_print("Unable to create include directory!");
        err_print(err);
        std::process::exit(1);
    }

    // Creating src/main.c
    if let Err(err) = create_file(
        src_path.to_str().unwrap(),
        "main.c",
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, CCraft!\\n\");\n    return 0;\n}",
    ) {
        err_print("Unable to create main.c!");
        err_print(err);
        std::process::exit(1);
    }

    let config = config::types::Config::default(project_name);
    if let Err(err) = create_file(project_directory, "craft.toml", &config.serialize()) {
        err_print("Unable to create craft.toml!");
        err_print(err);
        std::process::exit(1);
    }
}
