use super::{replace_files_contents, FileBumpError};
use std::collections::HashMap;

use crate::{config, core};
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
fn test_replace_files_contents_no_files() {
    let current_version = core::Version::with_values(1, 2, 3, None);
    let new_version = core::Version::with_values(4, 5, 6, None);
    let last_stable_version = core::Version::with_values(1, 2, 3, None);

    let files = HashMap::new();
    assert_eq!(
        replace_files_contents(
            &current_version,
            &new_version,
            Some(&last_stable_version),
            &files
        ),
        Ok(())
    );
}

#[test]
fn test_replace_files_contents_version_not_found() {
    let func_name = "test_replace_files_contents_version_not_found";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let last_stable_version = core::Version::with_values(1, 2, 3, None);
        let wrong_version = core::Version::with_values(4, 5, 6, None);
        let file_paths = HashMap::from([
            (
                "file-1",
                create_versioned_file(test_dir_name, "file-1", &current_version.to_string())
                    .unwrap(),
            ),
            (
                "file-2",
                create_versioned_file(test_dir_name, "file-2", &wrong_version.to_string()).unwrap(),
            ),
            (
                "file-3",
                create_versioned_file(test_dir_name, "file-3", &current_version.to_string())
                    .unwrap(),
            ),
        ]);

        let mut files = HashMap::new();
        for val in file_paths.values() {
            files.insert(val.to_owned(), config::FileConfig::new());
        }
        assert_eq!(
            replace_files_contents(
                &current_version,
                &new_version,
                Some(&last_stable_version),
                &files
            ),
            Err(FileBumpError::NoOp(format!(
                "Nothing changed in file '{}'",
                file_paths.get("file-2").unwrap(),
            )))
        );
    });
}

#[test]
fn test_replace_files_contents_some_file_not_exists() {
    let func_name = "test_replace_files_contents_some_file_not_exists";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let last_stable_version = core::Version::with_values(1, 2, 3, None);
        let file_paths = HashMap::from([
            (
                "file-1",
                create_versioned_file(test_dir_name, "file-1", &current_version.to_string())
                    .unwrap(),
            ),
            (
                "file-2",
                format!("{}/{}", test_dir_name, "test-file_file-2"),
            ),
            (
                "file-3",
                create_versioned_file(test_dir_name, "file-3", &current_version.to_string())
                    .unwrap(),
            ),
        ]);

        let mut files = HashMap::new();
        for val in file_paths.values() {
            files.insert(val.to_owned(), config::FileConfig::new());
        }

        matches!(
            replace_files_contents(
                &current_version,
                &new_version,
                Some(&last_stable_version),
                &files,
            )
            .unwrap_err(),
            FileBumpError::Io(_)
        );
    });
}

#[test]
fn test_replace_files_contents() {
    let func_name = "test_replace_files_contents";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let last_stable_version = core::Version::with_values(1, 2, 3, None);
        let files = HashMap::from(["file-1", "file-2", "file-3"].map(|f| {
            (
                create_versioned_file(test_dir_name, f, &current_version.to_string()).unwrap(),
                config::FileConfig::new(),
            )
        }));
        assert_eq!(
            replace_files_contents(
                &current_version,
                &new_version,
                Some(&last_stable_version),
                &files
            ),
            Ok(())
        );
        assert!(files.keys().all(|fp| {
            let content = fs::read_to_string(fp).unwrap();
            content.contains(&format!("Version: '{}'", &new_version.to_string()))
        }));
    });
}

#[test]
fn test_replace_files_contents_with_file_config() {
    let func_name = "test_replace_files_contents_with_file_config";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let last_stable_version = core::Version::with_values(1, 2, 3, None);
        let files = HashMap::from(["file-1", "file-2", "file-3"].map(|f| {
            (
                create_versioned_file(test_dir_name, f, &current_version.to_string()).unwrap(),
                config::FileConfig::with_pattern(
                    r#"Version: '{current_version}'"#.to_string(),
                    r"__VERSION__ = '{new_version}'".to_string(),
                ),
            )
        }));
        assert_eq!(
            replace_files_contents(
                &current_version,
                &new_version,
                Some(&last_stable_version),
                &files
            ),
            Ok(())
        );
        assert!(files.keys().all(|fp| {
            let content = fs::read_to_string(fp).unwrap();
            content.contains(&format!("__VERSION__ = '{}'", &new_version.to_string()))
        }));
    });
}

#[test]
fn test_replace_files_contents_with_file_config_stable_only() {
    let func_name = "test_replace_files_contents_with_file_config_stable_only";
    with_test_dir(func_name, |test_dir_name| {
        let current_version = core::Version::with_values(1, 2, 3, None);
        let new_version = core::Version::with_values(1, 2, 3, Some("dev.1".to_owned()));
        let last_stable_version = core::Version::with_values(1, 2, 3, None);
        let files = HashMap::from(["file-1", "file-2", "file-3"].map(|f| {
            (
                create_versioned_file(test_dir_name, f, &current_version.to_string()).unwrap(),
                config::FileConfig::with_params(
                    r#"Version: '{current_version}'"#.to_string(),
                    r"__VERSION__ = '{new_version}'".to_string(),
                    true,
                ),
            )
        }));
        assert_eq!(
            replace_files_contents(
                &current_version,
                &new_version,
                Some(&last_stable_version),
                &files
            ),
            Ok(())
        );
        assert!(files.keys().all(|fp| {
            let content = fs::read_to_string(fp).unwrap();
            content.contains(&format!("Version: '{}'", &current_version.to_string()))
        }));
    });
}
