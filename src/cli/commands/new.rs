use std::io::{stdin, stdout, Write};

use crate::utils::folder::{
    create_folder, is_folder_exists,
    validate_folder_name,
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
    let res = create_folder(project_name);

    if res.is_err() {
        eprintln!("Unable to create folder!\n{}", res.unwrap_err());
        return;
    }
    println!("Project successfully created!");
}
