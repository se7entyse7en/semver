use clap::{Parser, Subcommand};
pub mod error;
pub mod helpers;
pub mod validate;
use validate::ValidateArgs;
pub mod bump;
use bump::BumpArgs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Checks if the provided version is semver compliant
    Validate(ValidateArgs),

    /// Bumps the version in a file
    Bump(BumpArgs),
}
