use crate::config::{
    loader::{load_config, validate},
    types::Config,
};
use std::process::Command;

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
    for flag in config.get_build().get_flags() {
        build_command.arg(flag);
    }
    let a = build_command.output().unwrap();
    println!("{:?}", a);
    println!("{:?}", config);
    // TODO: Complete
}
