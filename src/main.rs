#[macro_use]
extern crate lazy_static;


use clap::Parser;

mod args;
mod commands;
mod build_system;

fn main() {
    let args = args::Args::parse();
    // Execute action based on command
    args.execute();
}
