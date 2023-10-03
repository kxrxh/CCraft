use std::{fs, io};

pub fn is_folder_exists(folder_path: &str) -> bool {
    fs::metadata(folder_path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

// Function to create a new folder with a validated name
pub fn create_folder(folder_path: &str) -> io::Result<()> {
    if validate_folder_name(folder_path) {
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
#[inline(always)]
pub fn validate_folder_name(folder_name: &str) -> bool {
    if folder_name.is_empty() {
        return false;
    }
    folder_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}
