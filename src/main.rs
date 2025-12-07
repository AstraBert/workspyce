mod check;

use crate::check::{check};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CliArgs {
     #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// check the current status of the packages based 
    /// on git status.
    /// If an untracked modification is detected, you will be prompted
    /// to provide a version bump for the involved package (or ignore the change).
    /// 
    /// Changes for each package will be saved in a markdown file under `.workspyce/``
    Check {
        /// The path to the pyproject file containing uv workspace details
        #[arg(short = 'p', long ="pyproject")]
        path: String,
    },
}

fn main() {
    let args = CliArgs::parse();
    match args.cmd {
        Commands::Check { path } => check(&path) 
    }
}
