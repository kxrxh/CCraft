use clap::Parser;

mod cli;
mod utils;
mod config;

fn main() {
    let args = cli::types::Args::parse();
    // Execute action based on command
    args.execute();
}
