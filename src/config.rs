#[cfg(test)]
mod tests;
use crate::core;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::{fmt, fs, io, str};
use toml;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct WrapperRawConfig {
    semver: RawConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RawConfig {
    current_version: String,
    last_stable_version: Option<String>,
    default_part: String,
    files: Option<HashMap<String, FileConfig>>,
    prerelease: Option<PrereleaseConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileConfig {
    pub search: Option<String>,
    pub replace: Option<String>,
    pub stable_only: Option<bool>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl FileConfig {
    pub fn new() -> Self {
        FileConfig {
            search: None,
            replace: None,
            stable_only: None,
        }
    }

    pub fn with_stable_only() -> Self {
        FileConfig {
            search: None,
            replace: None,
            stable_only: Some(true),
        }
    }

    pub fn with_pattern(search: String, replace: String) -> Self {
        FileConfig {
            search: Some(search),
            replace: Some(replace),
            stable_only: None,
        }
    }

    pub fn with_params(search: String, replace: String, stable_only: bool) -> Self {
        FileConfig {
            search: Some(search),
            replace: Some(replace),
            stable_only: Some(stable_only),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PrereleaseConfig {
    bump_script: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: Option<String>,
    pub current_version: String,
    pub last_stable_version: Option<String>,
    pub default_part: core::Part,
    pub files: HashMap<String, FileConfig>,
    pub bump_prerelease_func: Option<String>,
    raw_config: WrapperRawConfig,
}

impl From<WrapperRawConfig> for Config {
    fn from(wrapper_config: WrapperRawConfig) -> Self {
        let raw_config = wrapper_config.clone();
        Config {
            current_version: wrapper_config.semver.current_version,
            last_stable_version: wrapper_config.semver.last_stable_version,
            // TODO: Avoid `unwrap`
            default_part: core::Part::from_str(&wrapper_config.semver.default_part).unwrap(),
            files: wrapper_config
                .semver
                .files
                .map_or(HashMap::new(), |files| files),
            bump_prerelease_func: wrapper_config
                .semver
                .prerelease
                .map(|prerel| prerel.bump_script),
            path: None,
            raw_config,
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
        let mut config = Config::from_str(&content)
            .map(|mut config| {
                config.path = Some(file_path.to_string());
                config
            })
            .map_err(ConfigError::from)?;
        config.path = Some(file_path.to_owned());
        Ok(config)
    }

    pub fn update(&self, new_version: &core::Version) -> Result<Config, io::Error> {
        let mut raw_config = self.raw_config.to_owned();
        raw_config.semver.last_stable_version = if new_version.prerelease.is_some() {
            self.last_stable_version
                .to_owned()
                .or_else(|| Some(self.current_version.to_owned()))
        } else {
            Some(new_version.to_string())
        };

        raw_config.semver.current_version = new_version.to_string();
        // TODO: serialization to TOML doesn't preserve the order
        let serialized_config = toml::to_string_pretty(&raw_config).unwrap();
        match &self.path {
            Some(path) => fs::write(path, &serialized_config)?,
            None => (),
        }
        Ok(Config::from_str(&serialized_config).unwrap())
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
