#[cfg(test)]
mod tests;
use crate::validate;
use clap::ArgEnum;
use regex::{self, Regex};
use serde::Deserialize;
use std::{fmt, str};

const SUPPORTED_PATTERN: &str =
    r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)$";

#[derive(Debug, Eq, PartialEq)]
pub enum VersionError {
    InvalidVersion(String),
    UnsupportedVersion(String),
}

#[derive(Debug, Clone, Eq, PartialEq, ArgEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Part {
    Major,
    Minor,
    Patch,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Major => write!(f, "major"),
            Self::Minor => write!(f, "minor"),
            Self::Patch => write!(f, "patch"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self::with_values(0, 0, 0)
    }

    pub fn with_values(major: usize, minor: usize, patch: usize) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    fn extract_part(caps: &regex::Captures, part: &Part) -> usize {
        caps.name(&part.to_string())
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
    }

    pub fn bump(&self, part: &Part) -> Self {
        match part {
            Part::Major => self.bump_major(),
            Part::Minor => self.bump_minor(),
            Part::Patch => self.bump_patch(),
        }
    }

    pub fn bump_major(&self) -> Self {
        Self::with_values(self.major + 1, self.minor, self.patch)
    }

    pub fn bump_minor(&self) -> Self {
        Self::with_values(self.major, self.minor + 1, self.patch)
    }

    pub fn bump_patch(&self) -> Self {
        Self::with_values(self.major, self.minor, self.patch + 1)
    }
}

impl str::FromStr for Version {
    type Err = VersionError;

    fn from_str(raw_version: &str) -> Result<Self, Self::Err> {
        if !validate::validate(raw_version) {
            Err(VersionError::InvalidVersion(raw_version.to_owned()))
        } else {
            let re = Regex::new(SUPPORTED_PATTERN).unwrap();
            if !re.is_match(raw_version) {
                Err(VersionError::UnsupportedVersion(raw_version.to_owned()))
            } else {
                let caps = re.captures(raw_version).unwrap();

                Ok(Self::with_values(
                    Self::extract_part(&caps, &Part::Major),
                    Self::extract_part(&caps, &Part::Minor),
                    Self::extract_part(&caps, &Part::Patch),
                ))
            }
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
