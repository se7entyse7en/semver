use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet the user
    Hello(Hello),
    /// Checks if the provided version is semver compliant
    Validate(Validate),
    /// Computes next version
    Next(Next),
    /// Bumps the version in a file
    Bump(Bump),
}

#[derive(Args)]
struct Hello {}

#[derive(Args)]
struct Validate {
    /// Version to check
    version: String,
}

#[derive(Args)]
struct Next {
    /// Current version from which to compute the next one
    current_version: String,
    /// Which part of the version to bump
    part: String, // TODO: Make `part` an `enum` in `core`
}

#[derive(Args)]
struct Bump {
    /// Current version from which to compute the next one
    current_version: String,
    /// Which part of the version to bump
    part: String,
    /// File containing the version to bump
    file: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello(_) => {
            semver::hello();
            std::process::exit(0);
        }
        Commands::Validate(args) => {
            let version = &args.version;
            match semver::validate::validate(version) {
                true => {
                    println!("Version '{}' is valid!", version);
                    std::process::exit(0);
                }
                false => {
                    println!("Version '{}' is not valid!", version);
                    std::process::exit(1);
                }
            }
        }
        Commands::Next(args) => {
            let current_version = &args.current_version;
            let part = &args.part;
            match semver::next::next(current_version, part) {
                Ok(version) => {
                    println!("Next version: '{}'", version);
                    std::process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        Commands::Bump(args) => {
            let current_version = &args.current_version;
            let part = &args.part;
            let file = &args.file;
            match semver::bump::bump(current_version, part, file) {
                Ok(version) => {
                    println!("Bumped to version: '{}'", version);
                    std::process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }
}
