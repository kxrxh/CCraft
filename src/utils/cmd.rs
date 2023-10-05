/// Checks if a command exists in the system.
///
/// # Arguments
///
/// * `command` - The name of the command to check.
///
/// # Returns
///
/// Returns `true` if the command exists, `false` otherwise.
pub(crate) fn check_command_exists(command: &str) -> bool {
    std::process::Command::new("which")
        .arg(command)
        .status()
        .is_ok()
}