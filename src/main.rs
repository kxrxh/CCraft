use clap::Parser;

mod args;
mod commands;

fn main() {
    let args = args::Args::parse();
    // Execute action based on command
    args.execute();
}
