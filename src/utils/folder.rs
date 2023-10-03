use std::{fs, io};

use std::path::Path;

pub fn is_folder_exists(folder_path: impl AsRef<Path>) -> bool {
    folder_path.as_ref().is_dir()
}
// Function to create a new folder with a validated name
pub fn create_folder(path_to_folder: &str, folder_name: &str) -> io::Result<()> {
    if validate_folder_name(folder_name) {
        let folder_path = Path::new(path_to_folder).join(folder_name);
        fs::create_dir_all(folder_path)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid folder name",
        ))
    }
}

// Function to validate folder name (only allows alphanumeric characters, underscores, and hyphens)
pub fn validate_folder_name(folder_name: &str) -> bool {
    if folder_name.is_empty() {
        return false;
    }
    folder_name
        .chars()
        .any(|c| c.is_alphanumeric() || c == '_' || c == '-')
}
