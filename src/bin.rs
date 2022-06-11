use clap::{self, Parser};
use semver as sv;
use sv::cmd::{bump, validate};
use sv::cmd::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate(args) => {
            validate::cli::validate(args);
        }
        Commands::Bump(args) => {
            bump::cli::bump(args);
        }
    }
}
