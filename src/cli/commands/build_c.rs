use crate::{
    config::{
        loader::{load_config, validate},
        types::Config,
    },
    utils::{
        file::search_files,
        folder::{create_folder, is_folder_exists},
    },
};
use std::process::Command;

const EXTENSIONS: [&str; 1] = ["c"];
pub(crate) fn build_project() {
    let config = load_config();
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    run_build(&config);
}

fn run_build(config: &Config) {
    validate(&config);
    let mut build_command = Command::new(config.get_build().get_compiler());
    let res = create_folder(".", "build");
    if res.is_err() {
        if !is_folder_exists("build") {
            eprintln!("Unable to create build folder");
            return;
        }
    }
    let files = search_files(&EXTENSIONS, "src");
    for file in files {
        build_command.arg(file);
    }
    build_command.arg("-o");
    build_command.arg(format!("build/{}", config.get_project().get_name()));
    for flag in config.get_build().get_flags() {
        build_command.arg(flag);
    }
    let a = build_command.output().unwrap();
    println!("{:?}", a);
    // TODO: Complete
}
