#[cfg(test)]
mod tests;
use crate::{core, validate};
use std::str::FromStr;

pub fn next(version: &str, part: &str) -> Result<core::Version, core::VersionError> {
    if !validate::validate(version) {
        Err(core::VersionError::InvalidVersion(version.to_owned()))
    } else {
        let v = core::Version::from_str(version)?;
        match part {
            "major" => Ok(v.bump_major()),
            "minor" => Ok(v.bump_minor()),
            "patch" => Ok(v.bump_patch()),
            _ => Err(core::VersionError::InvalidPart(part.to_owned())),
        }
    }
}
