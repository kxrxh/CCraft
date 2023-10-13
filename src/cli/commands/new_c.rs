use std::io::{BufRead, Write};

use crate::{
    config,
    utils::{
        self,
        file::create_file,
        folder::{create_folder, is_folder_exists, validate_folder_name},
        printer::{err_print, success_print, info_print},
    },
};

pub(crate) fn new_project_without_arg() {
    print!("Enter project name: ");
    std::io::stdout().flush().unwrap();

    let user_input = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap_or_else(|| {
            err_print("Unable to read project name");
            std::process::exit(1);
        })
        .unwrap_or_else(|_| {
            err_print("Unable to read project name");
            std::process::exit(1);
        })
        .trim()
        .to_string();

    new_project(user_input);
}

pub(crate) fn new_project(project_name: String) {
    // Validating project name
    if !validate_folder_name(&project_name) {
        err_print("Invalid folder name!\nPlease use only alphanumeric characters, underscores, and hyphens for project name");
        std::process::exit(1);
    }

    // Checking if folder exists. If it does, check if it is empty.
    if is_folder_exists(&project_name) {
        err_print("Directory already exists!");
        std::process::exit(1);
    }

    let time = std::time::Instant::now();
    // Creating folder
    info_print(format!("Creating new project `{}`", &project_name));

    let res = create_folder(".", &project_name);

    if res.is_err() {
        let message = format!("Unable to create folder!\n{:?}", res.unwrap_err());
        err_print(message);
        std::process::exit(1);
    }

    // Project folder was created
    // Now creating default project files
    fill_project_defaults(&project_name, &project_name);

    success_print(format!(
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
pub fn fill_project_defaults(project_folder: &str, project_name: &str) {
    // Creating README.md
    if let Err(err) = create_file(project_folder, "README.MD", format!("# {}", project_name)) {
        err_print("Unable to create README.MD!");
        err_print(err);
        std::process::exit(1);
    }

    // Creating src folder
    let src_path = std::path::PathBuf::from(project_folder).join("src");
    if let Err(err) = utils::folder::create_folder(project_folder, "src") {
        err_print("Unable to create src folder!");
        err_print(err);
        std::process::exit(1);
    }

    // Creating src/main.c

    if let Err(err) = create_file(
        src_path.to_str().unwrap(),
        "main.c",
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}"
            .to_string(),
    ) {
        err_print("Unable to create main.c!");
        err_print(err);
        std::process::exit(1);
    }

    let config = config::types::Config::default(project_name);
    if let Err(err) = create_file(project_folder, "config.toml", config.serialize()) {
        err_print("Unable to create config.toml!");
        err_print(err);
        std::process::exit(1);
    }
}
