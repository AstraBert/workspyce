mod check;
mod version;
mod release;

use crate::check::{check};
use crate::version::{version};
use crate::release::{release};
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
    /// bump the version of the packages in the workspace
    /// that have been modified and recorded within changelog files 
    /// (in the `.workspyce` directory) with the `check` command.
    Version {},
    /// release all the packages that were version-bumped.
    /// Run only after `check` and `version` both successfully completed.
    Release {
        /// Pypi token for publishing the packages
        #[arg(short = 't', long ="token")]
        token: String,
    }
}

fn main() {
    let args = CliArgs::parse();
    match args.cmd {
        Commands::Check { path } => check(&path),
        Commands::Version {} => version(),
        Commands::Release { token } => {
            match release(&token) {
                Ok(true) => {},
                Ok(false) => {},
                Err(e ) => {
                    eprintln!("{}", e)
                }
            }
        },
    }
}
