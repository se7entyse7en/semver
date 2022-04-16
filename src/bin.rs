use clap::{Args, CommandFactory, ErrorKind, Parser, Subcommand};
use semver as sv;
use std::convert::{identity, From};
use std::fmt;

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
    Hello(HelloArgs),
    /// Checks if the provided version is semver compliant
    Validate(ValidateArgs),
    /// Computes next version
    Next(NextArgs),
    /// Bumps the version in a file
    Bump(BumpArgs),
}

#[derive(Args)]
struct HelloArgs {}

#[derive(Args)]
struct ValidateArgs {
    /// Version to check
    version: String,
}

#[derive(Args)]
struct NextArgs {
    /// Current version from which to compute the next one
    #[clap(short = 'v', long)]
    current_version: Option<String>,
    /// Which part of the version to bump
    #[clap(short, long)]
    part: Option<String>, // TODO: Make `part` an `enum` in `core`
    /// Path of the configuration file
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Args)]
struct BumpArgs {
    /// Current version from which to compute the next one
    #[clap(short = 'v', long)]
    current_version: Option<String>,
    /// Which part of the version to bump
    #[clap(short, long)]
    part: Option<String>,
    /// File containing the version to bump
    #[clap(short, long)]
    file: Option<String>,
    /// Path of the configuration file
    #[clap(short, long)]
    config: Option<String>,
}

struct FinalizedNextArgs {
    current_version: String,
    part: String,
}

struct FinalizedBumpArgs {
    current_version: String,
    part: String,
    files: Vec<String>,
}

trait FinalizeArgs {
    type FinalizedArgs;

    fn get_config(&self) -> Option<String>;

    fn get_required_args(&self) -> Vec<String>;

    fn finalize_from_config(&self, config: sv::config::Config) -> Self::FinalizedArgs;

    fn finalize_from_self(&self) -> Option<Self::FinalizedArgs>;

    fn finalize(&self) -> Result<Self::FinalizedArgs, ArgumentsError> {
        match self.get_config().as_ref() {
            Some(config_path) => {
                let config = sv::config::Config::from_file(config_path)?;
                Ok(self.finalize_from_config(config))
            }
            None => {
                let required_args = self.get_required_args();
                self.finalize_from_self()
                    .ok_or(ArgumentsError::MissingArguments(required_args))
            }
        }
    }
}

impl FinalizeArgs for NextArgs {
    type FinalizedArgs = FinalizedNextArgs;

    fn get_config(&self) -> Option<String> {
        self.config.to_owned()
    }

    fn get_required_args(&self) -> Vec<String> {
        vec!["current_version".to_owned(), "part".to_owned()]
    }

    fn finalize_from_config(&self, config: sv::config::Config) -> Self::FinalizedArgs {
        FinalizedNextArgs {
            current_version: config.current_version,
            part: self.part.to_owned().map_or(config.default_part, identity),
        }
    }

    fn finalize_from_self(&self) -> Option<Self::FinalizedArgs> {
        match (self.current_version.as_ref(), self.part.as_ref()) {
            (Some(current_version), Some(part)) => Some(FinalizedNextArgs {
                current_version: current_version.to_owned(),
                part: part.to_owned(),
            }),
            _ => None,
        }
    }
}

impl FinalizeArgs for BumpArgs {
    type FinalizedArgs = FinalizedBumpArgs;

    fn get_config(&self) -> Option<String> {
        self.config.to_owned()
    }

    fn get_required_args(&self) -> Vec<String> {
        vec![
            "current_version".to_owned(),
            "part".to_owned(),
            "file".to_owned(),
        ]
    }

    fn finalize_from_config(&self, config: sv::config::Config) -> Self::FinalizedArgs {
        let mut files: Vec<String> = config.files.into_keys().collect();
        if let Some(config_path) = config.path {
            files.push(config_path);
        }

        FinalizedBumpArgs {
            current_version: config.current_version,
            part: self.part.to_owned().map_or(config.default_part, identity),
            files,
        }
    }

    fn finalize_from_self(&self) -> Option<Self::FinalizedArgs> {
        match (
            self.current_version.as_ref(),
            self.part.as_ref(),
            self.file.as_ref(),
        ) {
            (Some(current_version), Some(part), Some(file)) => Some(FinalizedBumpArgs {
                current_version: current_version.to_owned(),
                part: part.to_owned(),
                files: vec![file.to_owned()],
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ArgumentsError {
    ConfigError(sv::config::ConfigError),
    MissingArguments(Vec<String>),
}

impl From<sv::config::ConfigError> for ArgumentsError {
    fn from(err: sv::config::ConfigError) -> Self {
        ArgumentsError::ConfigError(err)
    }
}

impl fmt::Display for ArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentsError::ConfigError(err) => write!(f, "{}", err),
            ArgumentsError::MissingArguments(err) => write!(f, "{:#?}", err),
        }
    }
}

impl std::error::Error for ArgumentsError {}

fn handle_args_error(err: ArgumentsError) {
    match err {
        ArgumentsError::ConfigError(err) => {
            println!("Error while parsing configuration: {}", err);
            std::process::exit(2);
        }
        ArgumentsError::MissingArguments(missings) => {
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ArgumentConflict,
                format!(
                    "When configuration is not provided these are mandatory: {:#?}",
                    missings
                ),
            )
            .exit();
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello(_) => {
            sv::hello();
            std::process::exit(0);
        }
        Commands::Validate(args) => {
            let version = &args.version;
            match sv::validate::validate(version) {
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
            match args.finalize() {
                Ok(config) => match sv::next::next(&config.current_version, &config.part) {
                    Ok(version) => {
                        println!("Next version: '{}'", version);
                        std::process::exit(0);
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                },
                Err(err) => handle_args_error(err),
            };
        }
        Commands::Bump(args) => {
            match args.finalize() {
                Ok(config) => {
                    match sv::bump::bump(&config.current_version, &config.part, &config.files) {
                        Ok(version) => {
                            println!("Bumped to version: '{}'", version);
                            std::process::exit(0);
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    }
                }
                Err(err) => handle_args_error(err),
            };
        }
    }
}
