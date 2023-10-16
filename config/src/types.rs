use serde::{Deserialize, Serialize};

use super::defaults::{
    default_compiler, default_output_dir, default_project_type, default_source_dir, default_version,
};

/// Represents the configuration for the entire project.
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// Information about the project.
    project: ProjectInfo,
    /// Configuration for the build process.
    build: BuildConfig,
}

/// Represents project information.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectInfo {
    /// The name of the project.
    name: String,

    /// The version of the project. Defaults to "default_version" if not provided.
    #[serde(default = "default_version")]
    version: String,

    /// The description of the project. Can be None.
    description: Option<String>,

    /// The license of the project. Can be None.
    license: Option<String>,

    /// The authors of the project. Can be None or an empty Vec.
    authors: Option<Vec<String>>,
}
/// Represents the build configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    /// The type of the project.
    #[serde(default = "default_project_type")]
    project_type: String,

    /// The compiler to use for building.
    #[serde(default = "default_compiler")]
    compiler: String,

    /// Flags to be passed to the build process.
    build_flags: Vec<String>,

    /// Flags to be passed to the compilation process.
    compile_flags: Vec<String>,

    /// Directories to include in the build process.
    include_dirs: Vec<String>,

    /// The source directory for the project.
    #[serde(default = "default_source_dir", skip_serializing, skip_deserializing)]
    source_dir: String,

    /// The output directory for the build artifacts.
    #[serde(default = "default_output_dir", skip_serializing, skip_deserializing)]
    output_dir: String,
}

/// Represents a configuration object.
///
/// This struct holds information about the project and build configuration.
impl Config {
    /// Creates a new `Config` object with default values.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project.
    ///
    /// # Returns
    ///
    /// A new `Config` object with default values.
    pub fn default(name: &str) -> Self {
        Config {
            project: ProjectInfo::default(name),
            build: BuildConfig::default(),
        }
    }

    /// Serializes the `Config` object to a `String` using the `toml` library.
    ///
    /// # Returns
    ///
    /// A `String` representation of the `Config` object.
    pub fn serialize(&self) -> String {
        toml::to_string(self).unwrap()
    }

    /// Deserializes a `Config` object from a `String` using the `toml` library.
    ///
    /// # Arguments
    ///
    /// * `s` - The serialized `Config` object.
    ///
    /// # Returns
    ///
    /// A deserialized `Config` object.
    pub fn deserialize(s: &str) -> Self {
        toml::from_str(s).unwrap()
    }

    /// Returns a reference to the `ProjectInfo` object.
    ///
    /// # Returns
    ///
    /// A reference to the `ProjectInfo` object.
    pub fn get_project(&self) -> &ProjectInfo {
        &self.project
    }

    /// Returns a reference to the `BuildConfig` object.
    ///
    /// # Returns
    ///
    /// A reference to the `BuildConfig` object.
    pub fn get_build(&self) -> &BuildConfig {
        &self.build
    }
}

impl ProjectInfo {
    /// Creates a new `ProjectInfo` with the given name.
    /// Sets the default version to "0.1.0"
    /// Optional fields will be set to None
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project.
    ///
    /// # Returns
    ///
    /// A new `ProjectInfo` instance.
    pub fn default(name: &str) -> Self {
        ProjectInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            license: None,
            authors: None,
        }
    }

    /// Returns the name of the project.
    ///
    /// # Returns
    ///
    /// The name of the project as a string reference.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the version of the project.
    ///
    /// # Returns
    ///
    /// The version of the project as a string reference.
    pub fn get_version(&self) -> &str {
        &self.version
    }

    /// Returns the description of the project.
    ///
    /// # Returns
    ///
    /// The description of the project as an optional string reference.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the license of the project.
    ///
    /// # Returns
    ///
    /// The license of the project as an optional string reference.
    pub fn get_license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    /// Returns the authors of the project.
    ///
    /// # Returns
    ///
    /// The authors of the project as an optional vector of string references.
    pub fn get_authors(&self) -> Option<&Vec<String>> {
        self.authors.as_ref()
    }
}

/// Represents the configuration for building a project.
impl BuildConfig {
    /// Creates a new `BuildConfig` with default values.
    ///
    /// # Returns
    ///
    /// A new `BuildConfig` instance.
    pub fn default() -> Self {
        BuildConfig {
            project_type: default_project_type(),
            compiler: default_compiler(),
            build_flags: vec!["-Wall".to_string()],
            compile_flags: vec!["-Wall".to_string()],
            include_dirs: vec![],
            source_dir: default_source_dir(),
            output_dir: default_output_dir(),
        }
    }

    /// Returns a reference to the compiler used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the compiler.
    pub fn get_compiler(&self) -> &str {
        &self.compiler
    }

    /// Returns a reference to the project type of this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the project type.
    pub fn get_project_type(&self) -> &str {
        &self.project_type
    }

    /// Returns a reference to the build flags used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the build flags.
    pub fn get_build_flags(&self) -> &Vec<String> {
        &self.build_flags
    }

    /// Returns a reference to the compile flags used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the compile flags.
    pub fn get_compile_flags(&self) -> &Vec<String> {
        &self.compile_flags
    }

    /// Returns a reference to the include directories used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the include directories.
    pub fn get_include_dirs(&self) -> &Vec<String> {
        &self.include_dirs
    }

    /// Returns a reference to the source directory used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the source directory.
    pub fn get_source_dir(&self) -> &str {
        &self.source_dir
    }

    /// Returns a reference to the output directory used by this `BuildConfig`.
    ///
    /// # Returns
    ///
    /// A reference to the output directory.
    pub fn get_output_dir(&self) -> &str {
        &self.output_dir
    }
}
