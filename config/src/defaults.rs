pub(crate) fn default_version() -> String {
    "0.1.0".to_string()
}

pub(crate) fn default_compiler() -> String {
    "gcc".to_string()
}

pub(crate) fn default_output_dir() -> String {
    "build".to_string()
}

pub(crate) fn default_source_dir() -> String {
    "src".to_string()
}

pub(crate) fn default_project_type() -> String {
    "bin".to_string()
}

pub(crate) fn default_include_dir() -> Vec<String> {
    vec!["include".to_string()]
}

pub(crate) fn default_linker() -> String {
    return default_compiler();
}
