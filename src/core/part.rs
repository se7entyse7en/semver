use clap;
use serde::Deserialize;
use std::{fmt, str};

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
