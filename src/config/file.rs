use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(rename = "ProjectName")]
    project_name: String,
    #[serde(default = "gcc", rename = "Compiler")]
    compiler: String,
    #[serde(rename = "BuildFlags")]
    build_flags: Optional<Vec<String>>,
    #[serde(rename = "Dependencies")]
    dependencies: Optional<Dependencie>,
}

#[derive(Deserialize, Debug)]
struct Dependencie {
    // TODO!
}

impl Config {
    pub fn new(
        project_name: String,
        compiler: Optional<String>,
        build_flags: Optional<Vec<String>>,
        dependencies: Optional<Dependencie>,
    ) -> Self {
        if project_name.is_empty() {
            panic!("project_name cannot be empty");
        }
        not_implemented!();
    }
}
