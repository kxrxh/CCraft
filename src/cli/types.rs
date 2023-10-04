use clap::{Parser, Subcommand};

use super::commands::{
    build_c::build_project,
    new_c::{new_project, new_project_without_arg},
};

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
    #[command(name = "init", about = "Initialize new C project in current directory")]
    Init,
    #[command(
        name = "new",
        about = "Create a new C project in current directory. (Creates new folder)"
    )]
    New(NewCommandArgs),
    #[command(
        name = "config",
        about = "Commands connected with config file of C project"
    )]
    #[command(name = "sync", about = "Sync config file. Update dependencies.")]
    Sync,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct NewCommandArgs {
    #[arg(help = "Project name to create")]
    project_name: Option<String>,
}

impl Args {
    pub fn execute(&self) {
        match &self.command {
            Command::Run => todo!(),
            Command::Build => build_project(),
            Command::Init => todo!(),
            Command::New(args) => {
                if let Some(project_name) = &args.project_name {
                    new_project(project_name);
                } else {
                    new_project_without_arg();
                }
            }
            Command::Sync => todo!(),
        }
    }
}
