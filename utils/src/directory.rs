use std::{fs, io};

use std::path::Path;

/// Checks if a directory exists at the given path.
///
/// # Arguments
///
/// * `directory_path` - The path of the directory to check.
///
/// # Returns
///
/// Returns `true` if the directory exists, `false` otherwise.
pub fn is_directory_exists(directory_path: impl AsRef<Path>) -> bool {
    directory_path.as_ref().is_dir()
}

/// Create a directory at the specified path with the given directory name.
///
/// # Arguments
///
/// * `path_to_directory` - The path to the parent directory where the new directory will be created.
/// * `directory_name` - The name of the new directory.
///
/// # Returns
///
/// Returns `Ok(())` if the directory creation is successful, otherwise returns an `io::Error`.
pub fn create_directory(path_to_directory: &str, directory_name: &str) -> io::Result<()> {
    // Join the path to the directory and the directory name
    let directory_path = std::path::Path::new(path_to_directory).join(directory_name);

    // Create the directory at the specified path
    fs::create_dir_all(directory_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Validates a directory name.
///
/// # Arguments
///
/// * `directory_name` - The name of the directory to validate.
///
/// # Returns
///
/// Returns `true` if the directory name is valid, `false` otherwise.
pub fn validate_directory_name(directory_name: &str) -> bool {
    // Check if the directory name is empty
    !directory_name.is_empty()
        && Path::new(directory_name).components().all(|c| match c {
            std::path::Component::Normal(s) => s
                .to_str()
                .unwrap()
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            _ => false,
        })
}
