use clap::Parser;

mod cli;
mod utils;

fn main() {
    let args = cli::types::Args::parse();

    // Execute action based on command
    args.execute();
}
