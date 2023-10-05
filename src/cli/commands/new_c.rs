use std::io::{BufRead, Write};

use crate::{
    config,
    utils::{
        self,
        file::create_file,
        folder::{create_folder, is_folder_exists, validate_folder_name},
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
            eprintln!("Unable to read project name");
            std::process::exit(1);
        })
        .unwrap_or_else(|_| {
            eprintln!("Unable to read project name");
            std::process::exit(1);
        })
        .trim()
        .to_string();

    new_project(user_input);
}

pub(crate) fn new_project(project_name: String) {
    // Validating project name
    if !validate_folder_name(&project_name) {
        eprintln!("Invalid folder name!\nPlease use only alphanumeric characters, underscores, and hyphens for project name");
        return;
    }

    // Checking if folder exists. If it does, check if it is empty.
    if is_folder_exists(&project_name) {
        eprintln!("Folder already exists!");
        return;
    }

    // Creating folder
    println!("Creating new project: {}", &project_name);
    let res = create_folder(".", &project_name);

    if res.is_err() {
        eprintln!("Unable to create folder!\n{}", res.unwrap_err());
        return;
    }

    // Project folder was created
    // Now creating default project files
    fill_project_defaults(&project_name);

    println!("Project successfully created!");
}

/// Fill project defaults for a given project name.
///
/// # Arguments
///
/// * `project_name` - The name of the project.
///
fn fill_project_defaults(project_name: &str) {
    // Creating README.md
    if let Err(_) = create_file(project_name, "README.MD", format!("# {}", project_name)) {
        eprintln!("Unable to create README.md!");
        std::process::exit(1);
    }

    // Creating src folder
    let src_path = std::path::PathBuf::from(project_name).join("src");
    if let Err(_) = utils::folder::create_folder(project_name, "src") {
        eprintln!("Unable to create src folder!");
        std::process::exit(1);
    }

    // Creating src/main.c

    if let Err(_) = create_file(
        src_path.to_str().unwrap(),
        "main.c",
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}"
            .to_string(),
    ) {
        eprintln!("Unable to create src/main.c!");
        std::process::exit(1);
    }

    let config = config::types::Config::default(project_name);
    if let Err(_) = create_file(project_name, "config.toml", config.serialize()) {
        eprintln!("Unable to create config.toml!");
        std::process::exit(1);
    }
}
