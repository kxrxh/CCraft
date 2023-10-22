use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use git2::Repository;
use regex::Regex;

use crate::printer::{err_print, info_print, warn_print};

/// Checks if a command exists in the system.
///
/// # Arguments
///
/// * `command` - The name of the command to check.
///
/// # Returns
///
/// Returns `true` if the command exists, `false` otherwise.
pub fn check_command_exists(command: &str) -> bool {
    std::process::Command::new("which")
        .arg(command)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}

/// Initializes a new Git repository in the specified project directory.
///
/// # Arguments
///
/// * `project_path` - The path of the project directory.
///
/// # Returns
///
/// Returns `Ok(())` if the repository initialization is successful, otherwise returns an `git2::Error`.
pub fn init_git_repository(project_path: &str) -> Result<(), git2::Error> {
    // Initialize a new Git repository in the specified project directory
    Repository::init(project_path)?;

    Ok(())
}

/// Represents the type of a line (Error, Warning, Other).
enum LineType {
    Error,
    Warning,
    Other,
}

/// Executes a command and prints its output.
///
/// # Arguments
///
/// * `command` - A mutable reference to the `Command` to execute.
///
/// # Returns
///
/// * `Ok(())` if the command executed successfully.
/// * `Err(())` if there was an error executing the command.
pub fn execute_command(command: &mut Command) -> Result<(), ()> {
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
        return Err(());
    };

    Ok(())
}