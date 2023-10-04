use std::io::{stdin, stdout, Write};

use crate::{
    config,
    utils::{
        self,
        folder::{create_folder, is_folder_exists, validate_folder_name},
    },
};

pub(crate) fn new_project_without_arg() {
    let mut user_input = String::new();
    print!("Enter project name: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut user_input)
        .expect("Unable to read project name");
    user_input = user_input.trim().to_string();
    new_project(&user_input);
}

pub(crate) fn new_project(project_name: &str) {
    // Validating project name
    if !validate_folder_name(project_name) {
        eprintln!("Invalid folder name!\nPlease use only alphanumeric characters, underscores, and hyphens for project name");
        return;
    }

    // Checking if folder exists. If it does, check if it is empty.
    if is_folder_exists(project_name) {
        eprintln!("Folder already exists!");
        return;
    }

    // Creating folder
    println!("Creating new project: {}", project_name);
    let res = create_folder(".", project_name);

    if res.is_err() {
        eprintln!("Unable to create folder!\n{}", res.unwrap_err());
        return;
    }

    // Project folder was created
    // Now creating default project files
    if !fill_project_defaults(project_name) {
        return;
    }
    println!("Project successfully created!");
}

fn fill_project_defaults(project_name: &str) -> bool {
    // Creating README.md
    if let Err(_) = utils::file::create_file(
        project_name,
        "README.md",
        format!("# {}", project_name).as_str(),
    ) {
        eprintln!("Unable to create README.md!");
        return false;
    }

    // Creating src folder
    if let Err(_) = utils::folder::create_folder(project_name, "src") {
        eprintln!("Unable to create src folder!");
        return false;
    }

    // Creating src/main.c
    if let Err(_) = utils::file::create_file(
        project_name,
        "src/main.c",
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}",
    ) {
        eprintln!("Unable to create src/main.c!");
        return false;
    }

    let config = config::types::Config::default(project_name);
    if let Err(_) = utils::file::create_file(project_name, "config.toml", &config.serialize()) {
        eprintln!("Unable to create config.toml!");
        return false;
    }
    true
}
