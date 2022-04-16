#[cfg(test)]
mod tests;
use crate::core;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fmt, fs, io, str};
use toml;

#[derive(Deserialize, Debug)]
struct WrapperRawConfig {
    semver: RawConfig,
}

#[derive(Deserialize, Debug)]
struct RawConfig {
    current_version: String,
    default_part: core::Part,
    files: Option<HashMap<String, FileConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct FileConfig {}

pub struct Config {
    pub path: Option<String>,
    pub current_version: String,
    pub default_part: core::Part,
    pub files: HashMap<String, FileConfig>,
}

impl From<WrapperRawConfig> for Config {
    fn from(wrapper_config: WrapperRawConfig) -> Self {
        Config {
            current_version: wrapper_config.semver.current_version,
            default_part: wrapper_config.semver.default_part,
            files: wrapper_config
                .semver
                .files
                .map_or(HashMap::new(), |files| files),
            path: None,
        }
    }
}

impl str::FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(raw_config: &str) -> Result<Self, Self::Err> {
        let wrapper_config: WrapperRawConfig = toml::from_str(raw_config)?;
        Ok(Config::from(wrapper_config))
    }
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(file_path)?;
        Config::from_str(&content)
            .map(|mut config| {
                config.path = Some(file_path.to_string());
                config
            })
            .map_err(ConfigError::from)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    ParseError(toml::de::Error),
    IOError(io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ParseError(err) => write!(f, "{}", err),
            ConfigError::IOError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IOError(err)
    }
}
