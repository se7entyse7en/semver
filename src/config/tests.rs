use super::Config;
use crate::core::{CorePart, Part};
use std::str::FromStr;

#[test]
fn test_config() {
    let config = Config::from_str(
        r#"
[semver]
current_version = "1.0.0"
last_stable_version = "1.0.0"
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]

    [semver.files."test-2.txt"]

    [semver.files."test-3.txt"]
"#,
    )
    .unwrap();

    assert_eq!(config.current_version, "1.0.0");
    assert_eq!(config.default_part, Part::Core(CorePart::Minor));
    assert_eq!(config.path, None);
    assert_eq!(config.bump_prerelease_func, None);

    let mut files: Vec<String> = config.files.into_keys().collect();
    files.sort_unstable();
    assert_eq!(files, ["test-1.txt", "test-2.txt", "test-3.txt"]);
}

#[test]
fn test_config_error() {
    let config_1 = Config::from_str(
        r#"
[semver]
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]

    [semver.files."test-2.txt"]

    [semver.files."test-3.txt"]
"#,
    );

    let config_2 = Config::from_str(
        r#"
[semver]
current_version = "1.0.0"

  [semver.files]

    [semver.files."test-1.txt"]

    [semver.files."test-2.txt"]

    [semver.files."test-3.txt"]
"#,
    );

    for config in [config_1, config_2] {
        assert!(matches!(config, Err(_)));
    }
}

#[test]
fn test_config_no_files() {
    let config_1 = Config::from_str(
        r#"
[semver]
current_version = "1.0.0"
last_stable_version = "1.0.0"
default_part = "minor"
"#,
    )
    .unwrap();

    let config_2 = Config::from_str(
        r#"
[semver]
current_version = "1.0.0"
last_stable_version = "1.0.0"
default_part = "minor"

  [semver.files]
"#,
    )
    .unwrap();

    for config in [config_1, config_2] {
        assert_eq!(config.current_version, "1.0.0");
        assert_eq!(config.default_part, Part::Core(CorePart::Minor));
        assert_eq!(config.path, None);
        assert_eq!(config.bump_prerelease_func, None);

        let actual_files: Vec<String> = config.files.into_keys().collect();
        let expected_files: Vec<String> = vec![];

        assert_eq!(actual_files, expected_files);
    }
}

#[test]
fn test_config_with_prerelease() {
    let config = Config::from_str(
        r#"
[semver]
current_version = "1.0.0"
last_stable_version = "1.0.0"
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]

    [semver.files."test-2.txt"]

    [semver.files."test-3.txt"]

  [semver.prerelease]
  bump_script = '''
var PREFIX = "dev.";
function bump(version) {
  var counter = !version.prerelease ? 0 : parseInt(version.prerelease.slice(PREFIX.length));
  return `${PREFIX}${counter + 1}`;
}
'''
"#,
    )
    .unwrap();

    assert_eq!(config.current_version, "1.0.0");
    assert_eq!(config.default_part, Part::Core(CorePart::Minor));
    assert_eq!(config.path, None);
    assert_eq!(
        config.bump_prerelease_func,
        Some(
            r#"var PREFIX = "dev.";
function bump(version) {
  var counter = !version.prerelease ? 0 : parseInt(version.prerelease.slice(PREFIX.length));
  return `${PREFIX}${counter + 1}`;
}
"#
            .to_owned()
        )
    );

    let mut files: Vec<String> = config.files.into_keys().collect();
    files.sort_unstable();
    assert_eq!(files, ["test-1.txt", "test-2.txt", "test-3.txt"]);
}
