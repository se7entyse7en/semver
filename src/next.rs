#[cfg(test)]
mod tests;
use crate::{core, validate};
use std::str::FromStr;

pub fn next(version: &str, part: &core::Part) -> Result<core::Version, core::VersionError> {
    if !validate::validate(version) {
        Err(core::VersionError::InvalidVersion(version.to_owned()))
    } else {
        let v = core::Version::from_str(version)?;
        Ok(v.bump(part))
    }
}
