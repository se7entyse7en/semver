use super::{replace_version_in_files, FileBumpError};
use crate::core;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

const TEST_DIR_BASE_NAME: &str = "./__";

fn create_versioned_file(
    path_name: &str,
    file_name: &str,
    version: &str,
) -> Result<String, io::Error> {
    let path = PathBuf::from(format!("{}/test-file_{}", path_name, file_name));
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
fn test_replace_version_no_files() {
    let current_version = core::Version::with_values(1, 2, 3, None);
    let new_version = core::Version::with_values(4, 5, 6, None);
    let file_paths = vec![];
    assert_eq!(
        replace_version_in_files(&current_version, &new_version, &file_paths),
        Ok(())
    );
}

#[test]
fn test_replace_version_current_version_equal_new_version() {
    let current_version = core::Version::with_values(1, 2, 3, None);
    let new_version = core::Version::with_values(1, 2, 3, None);
    let file_paths = vec![];
    assert_eq!(
        replace_version_in_files(&current_version, &new_version, &file_paths),
        Err(FileBumpError::NoOp(format!(
            "New version is equal to current version: {}",
            current_version
        )))
    );
}

#[test]
fn test_replace_version_some_file_version_not_found() {
    let func_name = "test_replace_version_some_file_version_not_found";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let wrong_version = core::Version::with_values(4, 5, 6, None);
        let file_paths = vec![
            create_versioned_file(test_dir_name, "file-1", &current_version.to_string()).unwrap(),
            create_versioned_file(test_dir_name, "file-2", &wrong_version.to_string()).unwrap(),
            create_versioned_file(test_dir_name, "file-3", &current_version.to_string()).unwrap(),
        ];
        assert_eq!(
            replace_version_in_files(&current_version, &new_version, &file_paths),
            Err(FileBumpError::NoOp(format!(
                "version '{}' not found in file '{}'",
                current_version, file_paths[1],
            )))
        );
    });
}

#[test]
fn test_replace_version_some_file_not_exists() {
    let func_name = "test_replace_version_some_file_not_exists";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let file_paths = vec![
            create_versioned_file(test_dir_name, "file-1", &current_version.to_string()).unwrap(),
            format!("{}/{}", test_dir_name, "test-file_file-2"),
            create_versioned_file(test_dir_name, "file-3", &current_version.to_string()).unwrap(),
        ];
        matches!(
            replace_version_in_files(&current_version, &new_version, &file_paths).unwrap_err(),
            FileBumpError::Io(_)
        );
    });
}

#[test]
fn test_replace_version() {
    let func_name = "test_replace_version";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let file_paths = vec![
            create_versioned_file(test_dir_name, "file-1", &current_version.to_string()).unwrap(),
            create_versioned_file(test_dir_name, "file-2", &current_version.to_string()).unwrap(),
            create_versioned_file(test_dir_name, "file-3", &current_version.to_string()).unwrap(),
        ];
        assert_eq!(
            replace_version_in_files(&current_version, &new_version, &file_paths),
            Ok(())
        );
        assert!(file_paths.iter().all(|fp| {
            let content = fs::read_to_string(fp).unwrap();
            content.contains(&new_version.to_string())
        }));
    });
}
