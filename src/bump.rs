#[cfg(test)]
mod tests;
use crate::{core, next};
use std::{fs, io};

#[derive(Debug)]
pub enum BumpError {
    Io(io::Error),
    Version(core::VersionError),
    NoOp(String),
}

impl From<core::VersionError> for BumpError {
    fn from(err: core::VersionError) -> BumpError {
        BumpError::Version(err)
    }
}

impl From<io::Error> for BumpError {
    fn from(err: io::Error) -> BumpError {
        BumpError::Io(err)
    }
}

pub fn bump(version: &str, part: &str, file_path: &str) -> Result<core::Version, BumpError> {
    let new_version = next::next(version, part)?;
    let content = fs::read_to_string(file_path)?;
    let replaced_content = content.replace(version, &new_version.to_string());
    if content == replaced_content {
        Err(BumpError::NoOp(format!(
            "version '{}' not found in file '{}'",
            version, file_path
        )))
    } else {
        fs::write(file_path, replaced_content)?;
        Ok(new_version)
    }
}
