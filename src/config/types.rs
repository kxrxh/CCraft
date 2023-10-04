use serde::{Deserialize, Serialize};

use super::defaults::{default_compiler, default_output_dir, default_source_dir, default_version};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    project: ProjectInfo,
    build: BuildConfig,
    target: TargetConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectInfo {
    name: String,
    #[serde(default = "default_version")]
    version: String,
    description: Option<String>,
    license: Option<String>,
    authors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildConfig {
    #[serde(default = "default_compiler")]
    compiler: String,
    flags: Vec<String>,
    include_dirs: Vec<String>,
    #[serde(default = "default_source_dir", skip_serializing, skip_deserializing)]
    source_dir: String,
    #[serde(default = "default_output_dir", skip_serializing, skip_deserializing)]
    output_dir: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TargetConfig {
    name: String,
    r#type: String,
}

impl Config {
    pub fn default(name: &str) -> Self {
        Config {
            project: ProjectInfo::default(name),
            build: BuildConfig::default(),
            target: TargetConfig::default(name),
        }
    }

    pub fn serialize(&self) -> String {
        toml::to_string(self).unwrap()
    }

    pub fn deserialize(s: &str) -> Self {
        toml::from_str(s).unwrap()
    }

    pub fn get_project(&self) -> &ProjectInfo {
        &self.project
    }

    pub fn get_build(&self) -> &BuildConfig {
        &self.build
    }

    pub fn get_target(&self) -> &TargetConfig {
        &self.target
    }
}

impl ProjectInfo {
    pub fn default(name: &str) -> Self {
        ProjectInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            license: None,
            authors: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    pub fn get_authors(&self) -> Option<&Vec<String>> {
        self.authors.as_ref()
    }
}

impl BuildConfig {
    pub fn default() -> Self {
        BuildConfig {
            compiler: default_compiler(),
            flags: vec![],
            include_dirs: vec![],
            source_dir: default_source_dir(),
            output_dir: default_output_dir(),
        }
    }

    pub fn get_compiler(&self) -> &str {
        &self.compiler
    }

    pub fn get_flags(&self) -> &Vec<String> {
        &self.flags
    }

    pub fn get_include_dirs(&self) -> &Vec<String> {
        &self.include_dirs
    }

    pub fn get_source_dir(&self) -> &str {
        &self.source_dir
    }

    pub fn get_output_dir(&self) -> &str {
        &self.output_dir
    }
}

impl TargetConfig {
    pub fn default(name: &str) -> Self {
        TargetConfig {
            name: name.to_string(),
            r#type: "application".to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &str {
        &self.r#type
    }
}