use crate::cmd::validate;
use crate::core::{CorePart, ExtensionPart, Part, VersionError, SUPPORTED_PATTERN};
use regex::{self, Regex};
use serde::Serialize;
use std::{fmt, str};

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

    pub fn is_stable(&self) -> bool {
        self.get_extension_part(&ExtensionPart::Prerelease)
            .is_none()
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
