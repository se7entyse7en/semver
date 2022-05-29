use clap::{self, Args, Parser};
use semver as sv;
use sv::cmd::{bump, validate};
use sv::cmd::{Cli, Commands};

#[derive(Args)]
struct HelloArgs {}

#[derive(Args)]
struct NextArgs {
    /// Current version from which to compute the next one
    #[clap(short = 'v', long, display_order = 1)]
    current_version: Option<String>,

    /// Which part of the version to bump
    ///
    /// If the current version is a "prerelease", then this is ignored and a "prerelease" bump is assumed.
    #[clap(arg_enum, short, long, display_order = 2)]
    part: Option<sv::core::CorePart>,

    /// Starts a new prerelease for the provided part or increase the current prerelease.
    ///
    /// This is incompatible with `--finalize-prerelease`.
    #[clap(long, display_order = 3)]
    new_prerelease: bool,

    /// Finalize the current prerelease
    ///
    /// The argument `-p, --part <PART>` is ignored. This is incompatible with `---prerelease`.
    #[clap(long, display_order = 4)]
    finalize_prerelease: bool,

    /// Path of the configuration file
    #[clap(short, long, display_order = 5)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Commands::Hello(_) => {
        //     sv::hello();
        //     std::process::exit(0);
        // }
        Commands::Validate(args) => {
            validate::cli::validate(args);
        }
        Commands::Bump(args) => {
            bump::cli::bump(args);
        }
    }
}
