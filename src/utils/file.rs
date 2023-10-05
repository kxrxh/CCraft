use std::path::Path;

/// Creates a file in the specified folder with the given name and content.
///
/// # Arguments
///
/// * `folder_path` - The path to the folder where the file will be created.
/// * `file_name` - The name of the file to create.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// Returns `Ok(())` if the file was successfully created and written to, or an `std::io::Error` if there was an error.
pub(crate) fn create_file(
    folder_path: &str,
    file_name: &str,
    content: String,
) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(folder_path).join(file_name);
    std::fs::write(path, content)
}
/// Checks if a file exists in a given folder.
///
/// # Arguments
///
/// * `folder_path` - The path of the folder.
/// * `file_name` - The name of the file.
///
/// # Returns
///
/// Returns `true` if the file exists, `false` otherwise.
pub(crate) fn is_file_exist(folder_path: &str, file_name: &str) -> bool {
    let path = Path::new(folder_path).join(file_name);
    path.exists()
}

/// Searches for files with specific extensions in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `extensions` - A slice of strings representing the file extensions to search for.
/// * `path` - The path to the directory to search in.
///
/// # Returns
///
/// A vector of strings containing the paths of the found files.
pub(crate) fn search_files_with_ext(extensions: &[&str], path: &str) -> Vec<String> {
    let mut files = vec![];
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                files.extend(search_files_with_ext(extensions, &path.to_string_lossy()));
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    files.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }
    files
}
