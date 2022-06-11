mod args;
#[cfg(test)]
mod tests;
pub use args::ValidateArgs;
pub mod cli;
use crate::core::{CorePart, ExtensionPart, Part};
use regex::Regex;

// Ref: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
const PATTERN: &str = r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";
const PATTERN_MAJOR: &str = r"^(?P<major>0|[1-9]\d*)$";
const PATTERN_MINOR: &str = r"^(?P<minor>0|[1-9]\d*)$";
const PATTERN_PATCH: &str = r"^(?P<patch>0|[1-9]\d*)$";
const PATTERN_PRERELEASE: &str = r"^(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*)$";

#[allow(dead_code)]
const PATTERN_BUILDMETADATA: &str = r"^(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*)$";

pub fn validate(version: &str) -> bool {
    validate_part(version, None)
}

pub fn validate_part(s: &str, part: Option<&Part>) -> bool {
    let pattern = match part {
        Some(Part::Core(CorePart::Major)) => PATTERN_MAJOR,
        Some(Part::Core(CorePart::Minor)) => PATTERN_MINOR,
        Some(Part::Core(CorePart::Patch)) => PATTERN_PATCH,
        Some(Part::Extension(ExtensionPart::Prerelease)) => PATTERN_PRERELEASE,
        // TODO: No support for `buildmetadata` part yet
        _ => PATTERN,
    };
    let re = Regex::new(pattern).unwrap();
    re.is_match(s)
}
