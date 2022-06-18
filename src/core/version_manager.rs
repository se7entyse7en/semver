use crate::core::{BumpError, CorePart, ExtensionPart, Part, Version};

pub trait ExtensionBumpFunc: Fn(&Version) -> Result<String, BumpError> {}
impl<T> ExtensionBumpFunc for T where T: Fn(&Version) -> Result<String, BumpError> {}

trait Bump {
    type Value;

    fn bump(&self, version: &Version) -> Result<Self::Value, BumpError>;
}

struct CoreBumper<'a> {
    kind: &'a CorePart,
}

impl<'a> CoreBumper<'a> {
    pub fn new(kind: &'a CorePart) -> Self {
        Self { kind }
    }
}

impl<'a> Bump for CoreBumper<'a> {
    type Value = usize;

    fn bump(&self, version: &Version) -> Result<Self::Value, BumpError> {
        Ok(version.get_core_part(self.kind) + 1)
    }
}

struct ExtensionBumper<'a> {
    #[allow(dead_code)]
    kind: &'a ExtensionPart,
    bump_func: Box<dyn ExtensionBumpFunc>,
}

impl<'a> ExtensionBumper<'a> {
    pub fn new(bump_func: Box<dyn ExtensionBumpFunc>) -> Self {
        Self {
            kind: &ExtensionPart::Prerelease,
            bump_func,
        }
    }
}

impl<'a> Bump for ExtensionBumper<'a> {
    type Value = String;

    fn bump(&self, version: &Version) -> Result<Self::Value, BumpError> {
        (self.bump_func)(version)
    }
}

pub struct VersionManager<'a> {
    core_major_bumper: CoreBumper<'a>,
    core_minor_bumper: CoreBumper<'a>,
    core_patch_bumper: CoreBumper<'a>,
    extension_bumper: Option<ExtensionBumper<'a>>,
}

impl<'a> Default for VersionManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> VersionManager<'a> {
    pub fn new() -> Self {
        Self::with_extension_bump_func(None)
    }

    pub fn with_extension_bump_func(
        extension_bump_func: Option<Box<dyn ExtensionBumpFunc>>,
    ) -> Self {
        Self {
            core_major_bumper: CoreBumper::new(&CorePart::Major),
            core_minor_bumper: CoreBumper::new(&CorePart::Minor),
            core_patch_bumper: CoreBumper::new(&CorePart::Patch),
            extension_bumper: extension_bump_func.map(|func| ExtensionBumper::new(func)),
        }
    }

    pub fn bump(&self, version: &Version, part: &Part) -> Result<Version, BumpError> {
        match (part, &version.prerelease) {
            (&Part::Core(_), Some(_)) => Err(BumpError::InvalidOperation(format!(
                "Cannot bump part {} for version {}, it's a prerelease",
                part, version
            ))),
            (&Part::Extension(ExtensionPart::Prerelease), None) => {
                Err(BumpError::InvalidOperation(format!(
                    "Cannot bump prerelease for version {}, it's not a prerelease",
                    version
                )))
            }
            _ => self.single_part_bump(version, part),
        }
    }

    pub fn new_prerelease(&self, version: &Version, part: &CorePart) -> Result<Version, BumpError> {
        match version.prerelease {
            Some(_) => Err(BumpError::InvalidOperation(format!(
                "Cannot create a new prerelease for version {}, it's already a prerelease",
                version
            ))),
            None => self.single_part_bump(
                &self.single_part_bump(version, &Part::Core(part.to_owned()))?,
                &Part::Extension(ExtensionPart::Prerelease),
            ),
        }
    }

    pub fn finalize_prerelease(&self, version: &Version) -> Result<Version, BumpError> {
        match version.prerelease {
            Some(_) => Ok(Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: None,
            }),
            None => Err(BumpError::InvalidOperation(format!(
                "Cannot finalize version {}, nothing to finalize",
                version
            ))),
        }
    }

    fn single_part_bump(&self, version: &Version, part: &Part) -> Result<Version, BumpError> {
        match part {
            Part::Core(CorePart::Major) => Ok(Version {
                major: self.core_major_bumper.bump(version)?,
                minor: 0,
                patch: 0,
                prerelease: version.prerelease.to_owned(),
            }),
            Part::Core(CorePart::Minor) => Ok(Version {
                major: version.major,
                minor: self.core_minor_bumper.bump(version)?,
                patch: 0,
                prerelease: version.prerelease.to_owned(),
            }),
            Part::Core(CorePart::Patch) => Ok(Version {
                major: version.major,
                minor: version.minor,
                patch: self.core_patch_bumper.bump(version)?,
                prerelease: version.prerelease.to_owned(),
            }),
            Part::Extension(ExtensionPart::Prerelease) => match &self.extension_bumper {
                Some(ext_bumper) => {
                    let prerelease = Some(ext_bumper.bump(version)?);
                    Ok(Version {
                        major: version.major,
                        minor: version.minor,
                        patch: version.patch,
                        prerelease,
                    })
                }
                None => Err(BumpError::MissingBumpScript),
            },
        }
    }
}
