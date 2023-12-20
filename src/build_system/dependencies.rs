use std::{collections::HashMap, fs};

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"\#include\s+["<]([^">]+)["<]\s*"#).unwrap();
}

/// Extracts dependencies from a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
///
/// # Returns
///
/// A vector of strings containing the extracted dependencies.
pub fn extract_dependencies(file_path: &str) -> Vec<String> {
    // Read the contents of the file into a string.
    let contents: String = fs::read_to_string(file_path).expect("Unable to read file");

    // Use a regular expression to find and capture the dependencies.
    let dependencies: Vec<String> = RE
        .captures_iter(&contents)
        .map(|cap| cap[1].to_string())
        .collect();

    dependencies
}

/// Creates a dependency tree recursively for the given directory and file extensions.
///
/// # Arguments
///
/// * `directory` - The directory to search for files.
/// * `file_extensions` - The allowed file extensions to consider.
///
/// # Returns
///
/// A HashMap representing the dependency tree, where the keys are file names and the values
/// are the dependencies of each file.
fn create_dependency_tree_recursive(
    directory: &str,
    file_extensions: &[&str],
) -> HashMap<String, Vec<String>> {
    let mut dependency_tree: HashMap<String, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(directory).expect("Unable to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if file_extensions.contains(&extension.to_str().unwrap()) {
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        let dependencies = extract_dependencies(&path.to_str().unwrap());

                        dependency_tree.insert(file_name, dependencies);
                    }
                }
            } else if path.is_dir() {
                // Recursively process subdirectories
                let subdirectory = path.to_str().unwrap();
                let subdirectory_tree =
                    create_dependency_tree_recursive(subdirectory, file_extensions);
                dependency_tree.extend(subdirectory_tree);
            }
        }
    }
    dependency_tree
}

/// Create a dependency tree for the given source directory and file extensions.
///
/// This function takes in a source directory path and an array of file extensions.
/// It returns a HashMap where the keys are the names of the files in the source directory,
/// and the values are vectors of the names of the files that the key file depends on.
///
/// # Arguments
///
/// * `source_dir` - The path to the source directory.
/// * `file_extensions` - An array of file extensions to consider when building the dependency tree.
///
/// # Returns
///
/// A HashMap representing the dependency tree.
pub fn create_dependency_tree(
    source_dir: &str,
    file_extensions: &[&str],
) -> HashMap<String, Vec<String>> {
    create_dependency_tree_recursive(source_dir, file_extensions)
}