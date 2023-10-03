use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(name = "run", about = "Run the application")]
    Run,
    #[command(name = "build", about = "Build the application")]
    Build,
    Check,
    #[command(name = "init", about = "Initialize new C project in current directory")]
    Init,
    #[command(name = "new", about = "Create a new C project in current directory")]
    New,
    #[command(
        name = "config",
        about = "Commands connected with config file of C project"
    )]
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ConfigCommand {
    #[command(name = "sync", about = "Sync config file. Update dependencies.")]
    Sync,
    #[command(name = "clear", about = "Clear config file")]
    Clear,
}

impl Args {
    pub fn execute(&self) {
        match &self.command {
            Command::Run => todo!(),
            Command::Build => todo!(),
            Command::Check => todo!(),
            Command::Init => todo!(),
            Command::New => todo!(),
            Command::Config(args) => match args.command {
                ConfigCommand::Sync => todo!(),
                ConfigCommand::Clear => todo!(),
            },
        }
    }
}
