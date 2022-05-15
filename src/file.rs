#[cfg(test)]
mod tests;
use crate::core;
use std::{fs, io};

#[derive(Debug)]
pub enum FileBumpError {
    Io(io::Error),
    NoOp(String),
}

impl From<io::Error> for FileBumpError {
    fn from(err: io::Error) -> FileBumpError {
        FileBumpError::Io(err)
    }
}

impl PartialEq for FileBumpError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileBumpError::Io(_), _) => false,
            (_, FileBumpError::Io(_)) => false,
            (FileBumpError::NoOp(m1), FileBumpError::NoOp(m2)) => m1 == m2,
        }
    }
}

pub fn replace_version_in_files(
    current_version: &core::Version,
    new_version: &core::Version,
    file_paths: &[String],
) -> Result<(), FileBumpError> {
    if current_version.to_string() == new_version.to_string() {
        Err(FileBumpError::NoOp(format!(
            "New version is equal to current version: {}",
            new_version
        )))
    } else {
        let mut res: Vec<Result<(), FileBumpError>> = vec![];
        for file_path in file_paths {
            res.push(replace_version_in_file(
                current_version,
                new_version,
                file_path,
            ));
        }

        // TODO: In case of an error in one of the files, they should all be reverted.
        res.into_iter()
            .collect::<Result<Vec<()>, FileBumpError>>()
            .map(|_| ())
    }
}

fn replace_version_in_file(
    current_version: &core::Version,
    new_version: &core::Version,
    file_path: &str,
) -> Result<(), FileBumpError> {
    let content = fs::read_to_string(file_path)?;
    let replaced_content = content.replace(&current_version.to_string(), &new_version.to_string());
    if content == replaced_content {
        Err(FileBumpError::NoOp(format!(
            "version '{}' not found in file '{}'",
            current_version, file_path
        )))
    } else {
        fs::write(file_path, replaced_content)?;
        Ok(())
    }
}
