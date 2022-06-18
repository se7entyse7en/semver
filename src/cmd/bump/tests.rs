use super::{bump, GenericBumpError};
use crate::config::FileConfig;
use crate::core::{
    BumpError, CorePart, ExtensionBumpFunc, ExtensionPart, Part, Version, VersionError,
};
use crate::file::FileBumpError;
use crate::tests::{v1, v2, v3};
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use uuid::Uuid;

const TEST_DIR_BASE_NAME: &str = "./__";

fn create_versioned_file(path_name: &str, version: &str) -> Result<String, io::Error> {
    let id = Uuid::new_v4();
    let path = PathBuf::from(format!("{}/test-file-{}-{}", path_name, version, id));
    fs::File::create(&path)?;
    let abs_path = fs::canonicalize(&path)?;
    if !version.is_empty() {
        fs::write(&abs_path, format!("Version: '{}'", version))?;
    }
    match abs_path.to_str() {
        Some(path) => Ok(path.to_owned()),
        None => Err(io::Error::new(io::ErrorKind::Other, "Missing file path")),
    }
}

fn with_test_dir<F>(test_func_name: &str, test_func: F)
where
    F: Fn(&str),
{
    let test_dir_name = format!("{}{}", TEST_DIR_BASE_NAME, test_func_name);
    fs::create_dir_all(&test_dir_name).unwrap();
    test_func(&test_dir_name);
    fs::remove_dir_all(test_dir_name).unwrap();
}

fn get_bump_prerelease_func() -> Box<dyn ExtensionBumpFunc> {
    Box::new(|v: &Version| {
        Ok(v.prerelease.as_ref().map_or("dev.1".to_owned(), |value| {
            let (prefix, num) = value.split_at(4);
            let num_inc = num.parse::<usize>().unwrap() + 1;
            format!("{}{}", prefix, num_inc)
        }))
    })
}

struct TestCaseBump {
    version: Version,
    part: Part,
    expected: Result<Version, GenericBumpError>,
}

impl TestCaseBump {
    pub fn new(version: Version, part: Part, expected: Result<Version, GenericBumpError>) -> Self {
        TestCaseBump {
            version,
            part,
            expected,
        }
    }
}

struct TestCaseBumpFinalize {
    version: Version,
    expected: Result<Version, GenericBumpError>,
}

impl TestCaseBumpFinalize {
    pub fn new(version: Version, expected: Result<Version, GenericBumpError>) -> Self {
        TestCaseBumpFinalize { version, expected }
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
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(format!(
                    "Cannot bump prerelease for version {}, it's not a prerelease",
                    v1()
                )),
            ))),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Major),
            Ok(Version::with_values(2, 0, 0, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Minor),
            Ok(Version::with_values(1, 3, 0, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Patch),
            Ok(Version::with_values(1, 2, 4, None)),
        ),
        TestCaseBump::new(
            v2(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump prerelease for version 1.2.3, it's not a prerelease".to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Major),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump part major for version 30.20.10-dev.5, it's a prerelease"
                        .to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Minor),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump part minor for version 30.20.10-dev.5, it's a prerelease"
                        .to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Patch),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump part patch for version 30.20.10-dev.5, it's a prerelease"
                        .to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v3(),
            Part::Extension(ExtensionPart::Prerelease),
            if with_extension {
                Ok(Version::with_values(30, 20, 10, Some("dev.6".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
    ]
}

fn get_test_cases_bump_new_prerelease(with_extension: bool) -> Vec<TestCaseBump> {
    vec![
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Major),
            if with_extension {
                Ok(Version::with_values(1, 0, 0, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Minor),
            if with_extension {
                Ok(Version::with_values(0, 1, 0, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v1(),
            Part::Core(CorePart::Patch),
            if with_extension {
                Ok(Version::with_values(0, 0, 1, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v1(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump prerelease part for a new prerelease".to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Major),
            if with_extension {
                Ok(Version::with_values(2, 0, 0, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Minor),
            if with_extension {
                Ok(Version::with_values(1, 3, 0, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v2(),
            Part::Core(CorePart::Patch),
            if with_extension {
                Ok(Version::with_values(1, 2, 4, Some("dev.1".to_owned())))
            } else {
                Err(GenericBumpError::Version(VersionError::BumpError(
                    BumpError::MissingBumpScript,
                )))
            },
        ),
        TestCaseBump::new(
            v2(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump prerelease part for a new prerelease".to_owned(),
                ),
            ))),
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Major),
                Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease"
                        .to_owned(),
                ),
            )))
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Minor),
                Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease"
                        .to_owned(),
                ),
            )))
        ),
        TestCaseBump::new(
            v3(),
            Part::Core(CorePart::Patch),
                Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot create a new prerelease for version 30.20.10-dev.5, it's already a prerelease"
                        .to_owned(),
                ),
            )))
        ),
        TestCaseBump::new(
            v3(),
            Part::Extension(ExtensionPart::Prerelease),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot bump prerelease part for a new prerelease".to_owned(),
                ),
            ))),
        ),
    ]
}

fn get_test_cases_bump_finalize_prerelease() -> Vec<TestCaseBumpFinalize> {
    vec![
        TestCaseBumpFinalize::new(
            v1(),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot finalize version 0.0.0, nothing to finalize".to_owned(),
                ),
            ))),
        ),
        TestCaseBumpFinalize::new(
            v2(),
            Err(GenericBumpError::Version(VersionError::BumpError(
                BumpError::InvalidOperation(
                    "Cannot finalize version 1.2.3, nothing to finalize".to_owned(),
                ),
            ))),
        ),
        TestCaseBumpFinalize::new(v3(), Ok(Version::with_values(30, 20, 10, None))),
    ]
}

mod test_with_extension {
    use super::{
        bump, create_versioned_file, get_bump_prerelease_func, get_test_cases_bump,
        get_test_cases_bump_finalize_prerelease, get_test_cases_bump_new_prerelease, with_test_dir,
        CorePart, FileConfig, Part,
    };
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_bump() {
        let func_name = "test_with_extension___test_bump";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump(true) {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(
                        &version,
                        None,
                        &tc.part,
                        false,
                        false,
                        &files,
                        Some(get_bump_prerelease_func()),
                    ),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }

    #[test]
    fn test_bump_new_prerelease() {
        let func_name = "test_with_extension___test_bump_new_prerelease";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump_new_prerelease(true) {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(
                        &version,
                        None,
                        &tc.part,
                        true,
                        false,
                        &files,
                        Some(get_bump_prerelease_func()),
                    ),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }

    #[test]
    fn test_bump_finalize_prerelease() {
        let func_name = "test_with_extension___test_bump_finalize_prerelease";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump_finalize_prerelease() {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(
                        &version,
                        None,
                        // TODO: Make `part` argument as non-required
                        &Part::Core(CorePart::Minor),
                        false,
                        true,
                        &files,
                        Some(get_bump_prerelease_func()),
                    ),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }
}

mod test_without_extension {
    use super::{
        bump, create_versioned_file, get_test_cases_bump, get_test_cases_bump_finalize_prerelease,
        get_test_cases_bump_new_prerelease, with_test_dir, CorePart, FileConfig, Part,
    };
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_bump() {
        let func_name = "test_without_extension___test_bump";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump(false) {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(&version, None, &tc.part, false, false, &files, None),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }

    #[test]
    fn test_bump_new_prerelease() {
        let func_name = "test_without_extension___test_bump_new_prerelease";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump_new_prerelease(false) {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(&version, None, &tc.part, true, false, &files, None,),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }

    #[test]
    fn test_bump_finalize_prerelease() {
        let func_name = "test_without_extension___test_bump_finalize_prerelease";
        with_test_dir(func_name, |test_dir_name| {
            for tc in get_test_cases_bump_finalize_prerelease() {
                let version = tc.version.to_string();
                let file_path = create_versioned_file(test_dir_name, &version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                assert_eq!(
                    bump(
                        &version,
                        None,
                        // TODO: Make `part` argument as non-required
                        &Part::Core(CorePart::Minor),
                        false,
                        true,
                        &files,
                        None,
                    ),
                    tc.expected
                );
                let file_content = fs::read_to_string(&file_path).unwrap();
                match tc.expected {
                    Ok(v) => assert_eq!(file_content, format!("Version: '{}'", v)),
                    _ => assert_eq!(file_content, format!("Version: '{}'", version)),
                }
            }
        });
    }
}

mod test_generic_errors {
    use super::{
        bump, create_versioned_file, get_bump_prerelease_func, with_test_dir, CorePart,
        FileBumpError, FileConfig, GenericBumpError, Part, VersionError,
    };
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_bump_invalid_version() {
        let func_name = "test_generic_errors___test_invalid_version";
        with_test_dir(func_name, |test_dir_name| {
            let test_cases_invalid = vec![
                ("00.01.00", &Part::Core(CorePart::Major)),
                ("1.2.3.dev1", &Part::Core(CorePart::Minor)),
                ("v1.2.3", &Part::Core(CorePart::Patch)),
                ("1", &Part::Core(CorePart::Major)),
            ];
            for tc in test_cases_invalid {
                let version = tc.0;
                let part = tc.1;
                let file_path = create_versioned_file(test_dir_name, version).unwrap();
                let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
                matches!(
                    bump(
                        version,
                        None,
                        part,
                        false,
                        false,
                        &files,
                        Some(get_bump_prerelease_func()),
                    )
                    .unwrap_err(),
                    GenericBumpError::Version(VersionError::InvalidVersion(_)),
                );
                assert_eq!(
                    fs::read_to_string(&file_path).unwrap(),
                    format!("Version: '{}'", version)
                )
            }
        });
    }

    #[test]
    fn test_bump_nothing_found() {
        let func_name = "test_generic_errors___test_bump_nothing_found";
        with_test_dir(func_name, |test_dir_name| {
            let expected_version = "1.0.0";
            let actual_version = "2.0.0";
            let file_path = create_versioned_file(test_dir_name, actual_version).unwrap();
            let files = HashMap::from([(file_path.to_owned(), FileConfig::new())]);
            matches!(
                bump(
                    expected_version,
                    None,
                    &Part::Core(CorePart::Major),
                    false,
                    false,
                    &files,
                    Some(get_bump_prerelease_func()),
                )
                .unwrap_err(),
                GenericBumpError::File(FileBumpError::NoOp(_))
            );
            assert_eq!(
                fs::read_to_string(&file_path).unwrap(),
                format!("Version: '{}'", actual_version)
            )
        });
    }

    #[test]
    fn test_bump_missing_file() {
        let func_name = "test_generic_errors___test_bump_missing_file";
        with_test_dir(func_name, |test_dir_name| {
            let expected_version = "1.0.0";
            let file_path = format!("{}/some-random-string-for-missing-test-file", test_dir_name);
            let files = HashMap::from([(file_path, FileConfig::new())]);
            matches!(
                bump(
                    expected_version,
                    None,
                    &Part::Core(CorePart::Major),
                    false,
                    false,
                    &files,
                    Some(get_bump_prerelease_func()),
                )
                .unwrap_err(),
                GenericBumpError::File(FileBumpError::Io(_))
            );
        })
    }
}
