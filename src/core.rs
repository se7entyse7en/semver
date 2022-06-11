#[cfg(test)]
mod tests;
use std::str;
mod version_manager;
pub use version_manager::{ExtensionBumpFunc, VersionManager};
mod version;
pub use version::Version;
mod part;
pub use part::{CorePart, ExtensionPart, Part};
mod error;
pub use error::{BumpError, VersionError};

const SUPPORTED_PATTERN: &str = r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?$";
