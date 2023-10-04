use std::{path::Path, fs::{File, self}, io::{Write, Error}};

pub(crate) fn create_file(folder_path: &str, file_name: &str, content: &str) -> Result<(), Error> {
    let path = Path::new(folder_path).join(file_name);
    let mut file = File::create(&path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

// Function to remove a file
pub(crate) fn remove_file(folder_path: &str, file_name: &str) -> Result<(), Error> {
    let path = Path::new(folder_path).join(file_name);
    fs::remove_file(path)?;
    Ok(())
}

pub(crate) fn is_file_exist(folder_path: &str, file_name: &str) -> bool {
    let path = Path::new(folder_path).join(file_name);
    path.exists()
}