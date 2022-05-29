mod args;
#[cfg(test)]
mod tests;

use crate::cmd::validate;
use crate::{core, file};
pub use args::{BumpArgs, FinalizedBumpArgs};
use std::str::FromStr;
pub mod cli;

#[derive(Debug, PartialEq)]
pub enum GenericBumpError {
    Version(core::VersionError),
    File(file::FileBumpError),
}

impl From<core::VersionError> for GenericBumpError {
    fn from(err: core::VersionError) -> GenericBumpError {
        GenericBumpError::Version(err)
    }
}

impl From<file::FileBumpError> for GenericBumpError {
    fn from(err: file::FileBumpError) -> GenericBumpError {
        GenericBumpError::File(err)
    }
}

pub fn bump(
    version: &str,
    part: &core::Part,
    new_prerelease: bool,
    finalize_prerelease: bool,
    file_paths: &[String],
    bump_prerelease_func: Option<Box<dyn core::ExtensionBumpFunc>>,
) -> Result<core::Version, GenericBumpError> {
    let new_version = next_version(
        version,
        part,
        new_prerelease,
        finalize_prerelease,
        bump_prerelease_func,
    )?;

    file::replace_version_in_files(&core::Version::from_str(version)?, &new_version, file_paths)
        .map(|()| new_version)
        .map_err(GenericBumpError::from)
}

fn next_version(
    version: &str,
    part: &core::Part,
    new_prerelease: bool,
    finalize_prerelease: bool,
    bump_prerelease_func: Option<Box<dyn core::ExtensionBumpFunc>>,
) -> Result<core::Version, core::VersionError> {
    if !validate::validate(version) {
        Err(core::VersionError::InvalidVersion(version.to_owned()))
    } else {
        let v = core::Version::from_str(version)?;
        let version_manager = core::VersionManager::with_extension_bump_func(bump_prerelease_func);
        if new_prerelease {
            match part {
                core::Part::Core(core_part) => version_manager
                    .new_prerelease(&v, core_part)
                    .map_err(core::VersionError::from),
                core::Part::Extension(_) => Err(core::VersionError::BumpError(
                    core::BumpError::InvalidOperation(
                        "Cannot bump prerelease part for a new prerelease".to_owned(),
                    ),
                )),
            }
        } else if finalize_prerelease {
            version_manager
                .finalize_prerelease(&v)
                .map_err(core::VersionError::from)
        } else {
            version_manager
                .bump(&v, part)
                .map_err(core::VersionError::from)
        }
    }
}
