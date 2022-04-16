use super::{bump, BumpError};
use crate::core::{Version, VersionError};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

const TEST_DIR_BASE_NAME: &str = "./__";

fn create_versioned_file(path_name: &str, version: &str) -> Result<String, io::Error> {
    let path = PathBuf::from(format!("{}/test-file-{}", path_name, version));
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

#[test]
fn test_bump() {
    let func_name = "test_bump";
    with_test_dir(func_name, |test_dir_name| {
        let valid = vec![
            ("0.0.0", "major", Version::with_values(1, 0, 0)),
            ("0.0.0", "minor", Version::with_values(0, 1, 0)),
            ("0.0.0", "patch", Version::with_values(0, 0, 1)),
            ("1.2.3", "major", Version::with_values(2, 2, 3)),
            ("1.2.3", "minor", Version::with_values(1, 3, 3)),
            ("1.2.3", "patch", Version::with_values(1, 2, 4)),
        ];
        for item in valid {
            let file_path = create_versioned_file(test_dir_name, item.0).unwrap();
            match bump(item.0, item.1, &[file_path]) {
                Ok(actual) => assert_eq!(actual, item.2),
                Err(err) => panic!(
                    "unexpected version error when bumping `{}` on version `{}`: {:?}",
                    item.1, item.0, err,
                ),
            }
        }
    });
}

#[test]
fn test_invalid_part() {
    let func_name = "test_invalid_part";
    with_test_dir(func_name, |test_dir_name| {
        let invalid_parts = vec![
            ("0.0.0", "a", VersionError::InvalidPart("a".to_owned())),
            ("0.0.0", "1", VersionError::InvalidPart("1".to_owned())),
            ("0.0.0", "", VersionError::InvalidPart("".to_owned())),
        ];
        for item in invalid_parts {
            let file_path = create_versioned_file(test_dir_name, item.0).unwrap();
            match bump(item.0, item.1, &[file_path]) {
                Ok(_) => panic!("part `{}` is surprisingly valid", item.1),
                Err(err) => match err {
                    BumpError::Version(version_error) => assert_eq!(version_error, item.2),
                    _ => panic!("unexpected error: {:?}", err),
                },
            }
        }
    });
}

#[test]
fn test_invalid_version() {
    let func_name = "test_invalid_version";
    with_test_dir(func_name, |test_dir_name| {
        let invalid_versions = vec![
            (
                "00.01.00",
                "major",
                VersionError::InvalidVersion("00.01.00".to_owned()),
            ),
            (
                "1.2.3.dev1",
                "minor",
                VersionError::InvalidVersion("1.2.3.dev1".to_owned()),
            ),
            (
                "v1.2.3",
                "patch",
                VersionError::InvalidVersion("v1.2.3".to_owned()),
            ),
            ("1", "major", VersionError::InvalidVersion("1".to_owned())),
        ];
        for item in invalid_versions {
            let file_path = create_versioned_file(test_dir_name, item.0).unwrap();
            match bump(item.0, item.1, &[file_path]) {
                Ok(_) => panic!("version `{}` is surprisingly valid", item.0),
                Err(err) => match err {
                    BumpError::Version(version_error) => assert_eq!(version_error, item.2),
                    _ => panic!("unexpected error: {:?}", err),
                },
            }
        }
    });
}

#[test]
fn test_bump_nothing_found() {
    let func_name = "test_bump_nothing_found";
    with_test_dir(func_name, |test_dir_name| {
        let expected_version = "1.0.0";
        let actual_version = "2.0.0";
        let file_path = create_versioned_file(test_dir_name, actual_version).unwrap();
        let file_paths = vec![file_path];
        match bump(expected_version, "major", &file_paths) {
            Ok(_) => panic!("version `{}` has been surprisingly found", expected_version),
            Err(err) => match err {
                BumpError::NoOp(version_error) => assert_eq!(
                    version_error,
                    format!(
                        "version '{}' not found in file '{}'",
                        expected_version, file_paths[0]
                    )
                ),
                _ => panic!("unexpected error: {:?}", err),
            },
        }
    });
}

#[test]
fn test_bump_missing_file() {
    let func_name = "test_bump_missing_file";
    with_test_dir(func_name, |test_dir_name| {
        let expected_version = "1.0.0";
        let file_path = format!("{}/some-random-string-for-missing-test-file", test_dir_name);
        let file_paths = vec![file_path];
        match bump(expected_version, "major", &file_paths) {
            Ok(_) => panic!("file `{}` has been surprisingly found", file_paths[0]),
            Err(err) => match err {
                BumpError::Io(_) => (),
                _ => panic!("unexpected error: {:?}", err),
            },
        }
    })
}
