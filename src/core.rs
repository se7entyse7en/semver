#[cfg(test)]
mod tests;
use crate::validate;
use anyhow;
use clap;
use regex::{self, Regex};
use serde::Deserialize;
use serde::Serialize;
use std::{fmt, num, str};
mod version_manager;
pub use version_manager::{ExtensionBumpFunc, VersionManager};

const SUPPORTED_PATTERN: &str = r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?$";

#[derive(Debug)]
pub enum VersionError {
    InvalidVersion(String),
    UnexpectedError(String),
    UnsupportedVersion(String),
    ParsingError(num::ParseIntError),
    BumpError(BumpError),
}

impl From<num::ParseIntError> for VersionError {
    fn from(err: num::ParseIntError) -> VersionError {
        VersionError::ParsingError(err)
    }
}

impl From<BumpError> for VersionError {
    fn from(err: BumpError) -> VersionError {
        VersionError::BumpError(err)
    }
}

#[derive(Debug)]
pub enum BumpError {
    AnyError(anyhow::Error),
    MissingBumpScript,
    InvalidOperation(String),
}

impl PartialEq for BumpError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BumpError::AnyError(_), _) => false,
            (_, BumpError::AnyError(_)) => false,
            (BumpError::MissingBumpScript, BumpError::MissingBumpScript) => true,
            (BumpError::InvalidOperation(m1), BumpError::InvalidOperation(m2)) => m1 == m2,
            _ => false,
        }
    }
}

impl From<anyhow::Error> for BumpError {
    fn from(err: anyhow::Error) -> BumpError {
        BumpError::AnyError(err)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Part {
    Core(CorePart),
    Extension(ExtensionPart),
}

#[derive(Debug)]
pub struct InvalidPartError {
    part: String,
}

impl fmt::Display for InvalidPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid part `{}`", self.part)
    }
}

impl std::error::Error for InvalidPartError {}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, clap::ArgEnum)]
#[serde(rename_all = "lowercase")]
pub enum CorePart {
    Major,
    Minor,
    Patch,
}

impl str::FromStr for CorePart {
    type Err = InvalidPartError;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        match part {
            "major" => Ok(CorePart::Major),
            "minor" => Ok(CorePart::Minor),
            "patch" => Ok(CorePart::Patch),
            _ => Err(InvalidPartError {
                part: part.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionPart {
    Prerelease,
}

impl str::FromStr for ExtensionPart {
    type Err = InvalidPartError;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        match part {
            "prerelease" => Ok(ExtensionPart::Prerelease),
            _ => Err(InvalidPartError {
                part: part.to_string(),
            }),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Core(CorePart::Major) => write!(f, "major"),
            Self::Core(CorePart::Minor) => write!(f, "minor"),
            Self::Core(CorePart::Patch) => write!(f, "patch"),
            Self::Extension(ExtensionPart::Prerelease) => write!(f, "prerelease"),
        }
    }
}

impl str::FromStr for Part {
    type Err = InvalidPartError;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        match part {
            "major" => Ok(Part::Core(CorePart::Major)),
            "minor" => Ok(Part::Core(CorePart::Minor)),
            "patch" => Ok(Part::Core(CorePart::Patch)),
            "prerelease" => Ok(Part::Extension(ExtensionPart::Prerelease)),
            _ => Err(InvalidPartError {
                part: part.to_string(),
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub prerelease: Option<String>,
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self::with_values(0, 0, 0, None)
    }

    pub fn with_values(
        major: usize,
        minor: usize,
        patch: usize,
        prerelease: Option<String>,
    ) -> Self {
        Version {
            major,
            minor,
            patch,
            prerelease,
        }
    }

    pub fn get_core_part(&self, part: &CorePart) -> usize {
        match part {
            CorePart::Major => self.major,
            CorePart::Minor => self.minor,
            CorePart::Patch => self.patch,
        }
    }

    pub fn get_extension_part(&self, part: &ExtensionPart) -> Option<String> {
        match part {
            ExtensionPart::Prerelease => self.prerelease.to_owned(),
        }
    }

    fn extract_part<T: str::FromStr>(caps: &regex::Captures, part: &Part) -> Option<T> {
        match caps.name(&part.to_string()) {
            Some(found) => found.as_str().parse::<T>().ok(),
            None => None,
        }
    }
}

impl str::FromStr for Version {
    type Err = VersionError;

    fn from_str(raw_version: &str) -> Result<Self, Self::Err> {
        if !validate::validate(raw_version) {
            Err(VersionError::InvalidVersion(format!(
                "Invalid version: {}",
                raw_version.to_owned()
            )))
        } else {
            let re = Regex::new(SUPPORTED_PATTERN).unwrap();
            if !re.is_match(raw_version) {
                Err(VersionError::UnsupportedVersion(format!(
                    "Unsupported version: {}",
                    raw_version.to_owned()
                )))
            } else {
                let caps = re.captures(raw_version).unwrap();
                let major = Self::extract_part::<usize>(&caps, &Part::Core(CorePart::Major))
                    .ok_or_else(|| {
                        VersionError::UnexpectedError(format!(
                            "Version {} is valid, but cannot extract 'major' part",
                            raw_version
                        ))
                    })?;
                let minor = Self::extract_part::<usize>(&caps, &Part::Core(CorePart::Minor))
                    .ok_or_else(|| {
                        VersionError::UnexpectedError(format!(
                            "Version {} is valid, but cannot extract 'minor' part",
                            raw_version
                        ))
                    })?;
                let patch = Self::extract_part::<usize>(&caps, &Part::Core(CorePart::Patch))
                    .ok_or_else(|| {
                        VersionError::UnexpectedError(format!(
                            "Version {} is valid, but cannot extract 'patch' part",
                            raw_version
                        ))
                    })?;
                let prerelease = Self::extract_part::<String>(
                    &caps,
                    &Part::Extension(ExtensionPart::Prerelease),
                );
                Ok(Self::with_values(major, minor, patch, prerelease))
            }
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.prerelease {
            Some(prerel) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, prerel),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}
