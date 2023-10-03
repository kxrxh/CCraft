use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    project: ProjectInfo,
    build: BuildConfig,
    target: TargetConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectInfo {
    name: String,
    version: String,
    description: Option<String>,
    license: Option<String>,
    authors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BuildConfig {
    compiler: String,
    flags: Vec<String>,
    include_dirs: Vec<String>,
    source_dir: String,
    output_dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TargetConfig {
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
}

impl BuildConfig {
    pub fn default() -> Self {
        BuildConfig {
            compiler: "gcc".to_string(),
            flags: vec![],
            include_dirs: vec![],
            source_dir: "src".to_string(),
            output_dir: "build".to_string(),
        }
    }
}

impl TargetConfig {
    pub fn default(name: &str) -> Self {
        TargetConfig {
            name: name.to_string(),
            r#type: "application".to_string(),
        }
    }
}