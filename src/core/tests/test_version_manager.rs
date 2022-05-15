use super::{v1, v2, v3};
use crate::core::{BumpError, CorePart, ExtensionPart, Part, Version, VersionManager};

struct TestCaseBump {
    version: Version,
    part: Part,
    expected: Result<Version, BumpError>,
}

impl TestCaseBump {
    pub fn new(version: Version, part: Part, expected: Result<Version, BumpError>) -> Self {
        TestCaseBump {
            version,
            part,
            expected,
        }
    }
}

struct TestCaseNewPrerelease {
    version: Version,
    part: CorePart,
    expected: Result<Version, BumpError>,
}

impl TestCaseNewPrerelease {
    pub fn new(version: Version, part: CorePart, expected: Result<Version, BumpError>) -> Self {
        TestCaseNewPrerelease {
            version,
            part,
            expected,
        }
    }
}

struct TestCaseFinalizePrerelease {
    version: Version,
    expected: Result<Version, BumpError>,
}

impl TestCaseFinalizePrerelease {
    pub fn new(version: Version, expected: Result<Version, BumpError>) -> Self {
        TestCaseFinalizePrerelease { version, expected }
    }
}

fn build_version_manager(with_extension: bool) -> VersionManager<'static> {
    if with_extension {
        VersionManager::with_extension_bump_func(Some(Box::new(|v: &Version| {
            Ok(v.prerelease.as_ref().map_or("dev.1".to_owned(), |value| {
                let (prefix, num) = value.split_at(4);
                let num_inc = num.parse::<usize>().unwrap() + 1;
                format!("{}{}", prefix, num_inc)
            }))
        })))
    } else {
        VersionManager::new()
    }
}

fn get_test_cases_bump(with_extension: bool) -> Vec<TestCaseBump> {
    vec![
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Major),
            Ok(Version::with_values(1, 0, 0, None)),
        ),
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Minor),
            Ok(Version::with_values(0, 1, 0, None)),
        ),
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Patch),
            Ok(Version::with_values(0, 0, 1, None)),
        ),
        TestCaseBump::new(
            v1(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(BumpError::InvalidOperation(
                "Cannot bump prerelease for version 0.0.0, it's not a prerelease".to_owned(),
            )),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Major),
            Ok(Version::with_values(2, 2, 3, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Minor),
            Ok(Version::with_values(1, 3, 3, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Patch),
            Ok(Version::with_values(1, 2, 4, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(BumpError::InvalidOperation(
                "Cannot bump prerelease for version 1.2.3, it's not a prerelease".to_owned(),
            )),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Major),
            Err(BumpError::InvalidOperation(
                "Cannot bump part major for version 30.20.10-dev.5, it's a prerelease".to_owned(),
            )),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Minor),
            Err(BumpError::InvalidOperation(
                "Cannot bump part minor for version 30.20.10-dev.5, it's a prerelease".to_owned(),
            )),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Patch),
            Err(BumpError::InvalidOperation(
                "Cannot bump part patch for version 30.20.10-dev.5, it's a prerelease".to_owned(),
            )),
        ),
        TestCaseBump::new(
            v3(),
            Part::Extension(ExtensionPart::Prerelease),
            if with_extension {
                Ok(Version::with_values(30, 20, 10, Some("dev.6".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
    ]
}

fn get_test_cases_new_prerelease(with_extension: bool) -> Vec<TestCaseNewPrerelease> {
    vec![
        TestCaseNewPrerelease::new(
            v1(),
            CorePart::Major,
            if with_extension {
                Ok(Version::with_values(1, 0, 0, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v1(),
            CorePart::Minor,
            if with_extension {
                Ok(Version::with_values(0, 1, 0, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v1(),
            CorePart::Patch,
            if with_extension {
                Ok(Version::with_values(0, 0, 1, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v2(),
            CorePart::Major,
            if with_extension {
                Ok(Version::with_values(2, 2, 3, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v2(),
            CorePart::Minor,
            if with_extension {
                Ok(Version::with_values(1, 3, 3, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v2(),
            CorePart::Patch,
            if with_extension {
                Ok(Version::with_values(1, 2, 4, Some("dev.1".to_owned())))
            } else {
                Err(BumpError::MissingBumpScript)
            },
        ),
        TestCaseNewPrerelease::new(
            v3(),
            CorePart::Major,
            Err(BumpError::InvalidOperation("Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease".to_owned()))
        ),
        TestCaseNewPrerelease::new(
            v3(),
            CorePart::Minor,
            Err(BumpError::InvalidOperation("Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease".to_owned()))
        ),
        TestCaseNewPrerelease::new(
            v3(),
            CorePart::Patch,
            Err(BumpError::InvalidOperation("Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease".to_owned()))
        ),
    ]
}

fn get_test_cases_finalize_prerelease() -> Vec<TestCaseFinalizePrerelease> {
    vec![
        TestCaseFinalizePrerelease::new(
            v1(),
            Err(BumpError::InvalidOperation(
                "Cannot finalize version 0.0.0, nothing to finalize".to_owned(),
            )),
        ),
        TestCaseFinalizePrerelease::new(
            v2(),
            Err(BumpError::InvalidOperation(
                "Cannot finalize version 1.2.3, nothing to finalize".to_owned(),
            )),
        ),
        TestCaseFinalizePrerelease::new(v3(), Ok(Version::with_values(30, 20, 10, None))),
    ]
}

mod test_without_extension {
    use super::{
        build_version_manager, get_test_cases_bump, get_test_cases_finalize_prerelease,
        get_test_cases_new_prerelease,
    };

    #[test]
    fn test_bump() {
        let vm = build_version_manager(false);

        for tc in get_test_cases_bump(false) {
            assert_eq!(vm.bump(&tc.version, &tc.part), tc.expected);
        }
    }

    #[test]
    fn test_new_prerelease() {
        let vm = build_version_manager(false);

        for tc in get_test_cases_new_prerelease(false) {
            assert_eq!(vm.new_prerelease(&tc.version, &tc.part), tc.expected);
        }
    }

    #[test]
    fn test_finalize_prerelease() {
        let vm = build_version_manager(false);

        for tc in get_test_cases_finalize_prerelease() {
            assert_eq!(vm.finalize_prerelease(&tc.version), tc.expected);
        }
    }
}

mod test_with_extension {
    use super::{
        build_version_manager, get_test_cases_bump, get_test_cases_finalize_prerelease,
        get_test_cases_new_prerelease,
    };

    #[test]
    fn test_bump() {
        let vm = build_version_manager(true);

        for tc in get_test_cases_bump(true) {
            assert_eq!(vm.bump(&tc.version, &tc.part), tc.expected);
        }
    }

    #[test]
    fn test_new_prerelease() {
        let vm = build_version_manager(true);

        for tc in get_test_cases_new_prerelease(true) {
            assert_eq!(vm.new_prerelease(&tc.version, &tc.part), tc.expected);
        }
    }

    #[test]
    fn test_finalize_prerelease() {
        let vm = build_version_manager(true);

        for tc in get_test_cases_finalize_prerelease() {
            assert_eq!(vm.finalize_prerelease(&tc.version), tc.expected);
        }
    }
}
