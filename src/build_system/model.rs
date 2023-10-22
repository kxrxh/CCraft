use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::time;
use utils::file::get_modification_time;

#[derive(Serialize, Deserialize)]
struct BuildCache {
    pub build_time: Vec<BuildTimeFileCache>,
}

#[derive(Serialize, Deserialize)]
struct BuildTimeFileCache {
    time: time::SystemTime,
    file: String,
    file_path: String,
    dependencies: Vec<String>,
}

impl BuildTimeFileCache {
    /// Creates a new `BuildTimeFileCache` from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON string representing the `BuildTimeFileCache`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `BuildCache` if successful, or an error if deserialization fails.
    pub fn new_from_json(json: &str) -> Result<BuildCache> {
        serde_json::from_str(json)
    }

    /// Converts the `BuildTimeFileCache` to a JSON string.
    ///
    /// # Returns
    ///
    /// The JSON string representing the `BuildTimeFileCache`.
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Creates a new `BuildTimeFileCache`.
    ///
    /// # Arguments
    ///
    /// * `file` - The file name.
    /// * `file_path` - The file path.
    /// * `dependencies` - The list of dependencies.
    ///
    /// # Returns
    ///
    /// The new `BuildTimeFileCache` instance.
    pub fn new(file: String, file_path: String, dependencies: Vec<String>) -> BuildTimeFileCache {
        BuildTimeFileCache {
            time: time::SystemTime::now(),
            file,
            file_path,
            dependencies,
        }
    }

    /// Returns the time of the `BuildTimeFileCache`.
    ///
    /// # Returns
    ///
    /// The time of the `BuildTimeFileCache`.
    pub fn get_time(&self) -> time::SystemTime {
        self.time
    }

    /// Returns the file name.
    ///
    /// # Returns
    ///
    /// The file name.
    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    /// Returns the file path.
    ///
    /// # Returns
    ///
    /// The file path.
    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }

    /// Returns the list of dependencies.
    ///
    /// # Returns
    ///
    /// The list of dependencies.
    pub fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    /// Sets the time of the `BuildTimeFileCache`.
    ///
    /// # Arguments
    ///
    /// * `time` - The new time.
    pub fn set_time(&mut self, time: time::SystemTime) {
        self.time = time
    }

    /// Checks if the `BuildTimeFileCache` is outdated.
    ///
    /// # Returns
    ///
    /// `true` if the cache is outdated, `false` otherwise.
    pub fn is_outdated(&self) -> bool {
        let current_modification_time = get_modification_time(&self.file_path);
        current_modification_time.unwrap_or_else(|| time::SystemTime::now()) > self.time
    }
}
