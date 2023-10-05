use std::{fs, io};

use std::path::Path;

/// Checks if a folder exists at the given path.
///
/// # Arguments
///
/// * `folder_path` - The path of the folder to check.
///
/// # Returns
///
/// Returns `true` if the folder exists, `false` otherwise.
pub fn is_folder_exists(folder_path: impl AsRef<Path>) -> bool {
    folder_path.as_ref().is_dir()
}


/// Create a folder at the specified path with the given folder name.
///
/// # Arguments
///
/// * `path_to_folder` - The path to the parent folder where the new folder will be created.
/// * `folder_name` - The name of the new folder.
///
/// # Returns
///
/// Returns `Ok(())` if the folder creation is successful, otherwise returns an `io::Error`.
pub fn create_folder(path_to_folder: &str, folder_name: &str) -> io::Result<()> {
    // Join the path to the folder and the folder name
    let folder_path = std::path::Path::new(path_to_folder).join(folder_name);

    // Create the folder at the specified path
    fs::create_dir_all(folder_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Validates a folder name.
///
/// # Arguments
///
/// * `folder_name` - The name of the folder to validate.
///
/// # Returns
///
/// Returns `true` if the folder name is valid, `false` otherwise.
pub fn validate_folder_name(folder_name: &str) -> bool {
    // Check if the folder name is empty
    !folder_name.is_empty()
        && Path::new(folder_name).components().all(|c| match c {
            std::path::Component::Normal(s) => s
                .to_str()
                .unwrap()
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            _ => false,
        })
}
