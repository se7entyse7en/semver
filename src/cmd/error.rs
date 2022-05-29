use crate::cmd::Cli;
use crate::config;
use clap::{CommandFactory, ErrorKind};
use std::{error, fmt};

#[derive(Debug)]
pub enum ArgumentsError {
    ConfigError(config::ConfigError),
    MissingArguments(Vec<String>),
}

impl From<config::ConfigError> for ArgumentsError {
    fn from(err: config::ConfigError) -> Self {
        ArgumentsError::ConfigError(err)
    }
}

impl fmt::Display for ArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentsError::ConfigError(err) => write!(f, "{}", err),
            ArgumentsError::MissingArguments(err) => write!(f, "{:#?}", err),
        }
    }
}

impl error::Error for ArgumentsError {}

pub fn handle_args_error(err: ArgumentsError) {
    let mut cmd = Cli::command();
    match err {
        ArgumentsError::ConfigError(err) => {
            println!("Error while parsing configuration: {}", err);
            std::process::exit(2);
        }
        ArgumentsError::MissingArguments(missings) => {
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
