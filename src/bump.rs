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

pub fn bump(version: &str, part: &str, file_paths: &[String]) -> Result<core::Version, BumpError> {
    let new_version = next::next(version, part)?;
    let mut res: Vec<Result<(), BumpError>> = vec![];
    for file_path in file_paths {
        res.push(bump_single(version, &new_version.to_string(), file_path));
    }

    res.into_iter()
        .collect::<Result<Vec<()>, BumpError>>()
        .map(|_| new_version)
}

fn bump_single(current_version: &str, new_version: &str, file_path: &str) -> Result<(), BumpError> {
    let content = fs::read_to_string(file_path)?;
    let replaced_content = content.replace(current_version, new_version);
    if content == replaced_content {
        Err(BumpError::NoOp(format!(
            "version '{}' not found in file '{}'",
            current_version, file_path
        )))
    } else {
        fs::write(file_path, replaced_content)?;
        Ok(())
    }
}
