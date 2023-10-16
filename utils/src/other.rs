use git2::Repository;

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