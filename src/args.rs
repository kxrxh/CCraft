use clap::{Parser, Subcommand};

use utils::printer::{err_print, info_print};

use crate::commands::{
    build_c::build_project,
    compile_c::compile_project,
    init_c::init,
    new_c::{get_project_name_from_user, new_project_bin},
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
    #[command(name = "compile", about = "Compile the application. Compilation only")]
    Compile,
    #[command(
        name = "build",
        about = "Build the application. Compilation and linking"
    )]
    Build,
    #[command(name = "init", about = "Initialize new C project in current directory")]
    Init,
    #[command(
        name = "new",
        about = "Create a new C project in current directory. (Creates new directory)"
    )]
    New(NewCommandArgs),
    #[command(
        name = "config",
        about = "Commands connected with config file of C project"
    )]
    #[command(name = "sync", about = "Sync config file. Update dependencies.")]
    Sync,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub(crate) struct NewCommandArgs {
    #[arg(help = "Project name to create")]
    project_name: Option<String>,
    #[arg(
        help = "Project type to create. Available types: `bin`, `static`, `shared`",
        short = 't',
        long = "type",
        default_value = "bin"
    )]
    project_type: Option<String>,
}

impl Args {
    pub fn execute(&self) {
        match &self.command {
            Command::Run => todo!(),
            Command::Compile => compile_project(),
            Command::Build => build_project(),
            Command::Init => {
                init();
            }
            Command::New(args) => {
                let type_name = args.project_type.as_deref().unwrap();

                if !["bin", "static", "shared"].contains(&type_name) {
                    err_print("Invalid project type!");
                    info_print("Available types: `bin`, `static`, `shared`. Please try again.");
                    std::process::exit(1);
                }

                let project_name = args
                    .project_name
                    .clone()
                    .unwrap_or_else(|| get_project_name_from_user());

                // Validate project type
                match type_name {
                    "bin" => new_project_bin(&project_name),
                    "static" => todo!(),
                    "shared" => todo!(),
                    _ => unreachable!(),
                }
            }
            Command::Sync => todo!(),
        }
    }
}
