use super::{Config, FileConfig};

mod test_config_update {
    use super::Config;
    use crate::core::Version;
    use std::str::FromStr;

    #[test]
    fn test_bump_stable_to_stable() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "1.0.0"
default_part = "minor"
"#,
        )
        .unwrap();
        let updated_config = config.update(&Version::with_values(2, 0, 0, None)).unwrap();
        assert_eq!(updated_config.current_version, "2.0.0".to_owned());
        assert_eq!(updated_config.last_stable_version, Some("2.0.0".to_owned()));
    }

    #[test]
    fn test_bump_stable_to_prerelease() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "1.0.0"
default_part = "minor"
"#,
        )
        .unwrap();
        let updated_config = config
            .update(&Version::with_values(1, 1, 0, Some("dev.1".to_owned())))
            .unwrap();
        assert_eq!(updated_config.current_version, "1.1.0-dev.1".to_owned());
        assert_eq!(updated_config.last_stable_version, Some("1.0.0".to_owned()));
    }

    #[test]
    fn test_bump_prerelease_to_stable() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "1.1.0-dev.1"
default_part = "minor"
"#,
        )
        .unwrap();
        let updated_config = config.update(&Version::with_values(2, 0, 0, None)).unwrap();
        assert_eq!(updated_config.current_version, "2.0.0".to_owned());
        assert_eq!(updated_config.last_stable_version, Some("2.0.0".to_owned()));
    }

    #[test]
    fn test_bump_prerelease_to_prerelease() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "1.1.0-dev.1"
default_part = "minor"
"#,
        )
        .unwrap();
        let updated_config = config
            .update(&Version::with_values(1, 1, 0, Some("dev.2".to_owned())))
            .unwrap();
        assert_eq!(updated_config.current_version, "1.1.0-dev.2".to_owned());
        // XXX: It assumes that the first `current_version` is a stable one. This could
        // be improved by avoiding this assumption and keep it to `None`.
        assert_eq!(
            updated_config.last_stable_version,
            Some("1.1.0-dev.1".to_owned())
        );
    }
}

mod test_config_parsing {
    use super::{Config, FileConfig};
    use crate::core::{CorePart, Part};
    use std::str::FromStr;

    #[test]
    fn test_wihout_prerelease() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "2.0.0"
last_stable_version = "1.0.0"
default_part = "minor"

[semver.files]

[semver.files."test-1.txt"]

[semver.files."test-2.txt"]

[semver.files."test-3.txt"]
search = "library = {current_version}"
replace = "library = {new_version}"
"#,
        )
        .unwrap();

        assert_eq!(config.current_version, "2.0.0");
        assert_eq!(config.last_stable_version, Some("1.0.0".to_owned()));
        assert_eq!(config.default_part, Part::Core(CorePart::Minor));
        assert_eq!(config.path, None);
        assert_eq!(config.bump_prerelease_func, None);

        let mut files: Vec<String> = config.files.keys().cloned().collect();
        files.sort_unstable();
        assert_eq!(files, ["test-1.txt", "test-2.txt", "test-3.txt"]);
        assert_eq!(
            config.files.get("test-1.txt"),
            Some(&FileConfig {
                search: None,
                replace: None,
            })
        );
        assert_eq!(
            config.files.get("test-2.txt"),
            Some(&FileConfig {
                search: None,
                replace: None,
            })
        );
        assert_eq!(
            config.files.get("test-3.txt"),
            Some(&FileConfig {
                search: Some("library = {current_version}".to_owned()),
                replace: Some("library = {new_version}".to_owned()),
            })
        );
    }

    #[test]
    fn test_error() {
        let config_1 = Config::from_str(
            r#"
[semver]
default_part = "minor"

[semver.files]

[semver.files."test-1.txt"]

[semver.files."test-2.txt"]

[semver.files."test-3.txt"]
search = "library = {current_version}"
replace = "library = {new_version}"
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
search = "library = {current_version}"
replace = "library = {new_version}"
"#,
        );

        for config in [config_1, config_2] {
            assert!(matches!(config, Err(_)));
        }
    }

    #[test]
    fn test_no_files() {
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

        let config_3 = Config::from_str(
            r#"
[semver]
current_version = "1.0.0"
default_part = "minor"
"#,
        )
        .unwrap();

        for config in [config_1, config_2, config_3] {
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
    fn test_with_prerelease() {
        let config = Config::from_str(
            r#"
[semver]
current_version = "2.0.0"
last_stable_version = "1.0.0"
default_part = "minor"

[semver.files]

[semver.files."test-1.txt"]

[semver.files."test-2.txt"]

[semver.files."test-3.txt"]
search = "library = {current_version}"
replace = "library = {new_version}"

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

        assert_eq!(config.current_version, "2.0.0");
        assert_eq!(config.last_stable_version, Some("1.0.0".to_owned()));
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

        let mut files: Vec<String> = config.files.keys().cloned().collect();
        files.sort_unstable();
        assert_eq!(files, ["test-1.txt", "test-2.txt", "test-3.txt"]);
        assert_eq!(
            config.files.get("test-1.txt"),
            Some(&FileConfig {
                search: None,
                replace: None,
            })
        );
        assert_eq!(
            config.files.get("test-2.txt"),
            Some(&FileConfig {
                search: None,
                replace: None,
            })
        );
        assert_eq!(
            config.files.get("test-3.txt"),
            Some(&FileConfig {
                search: Some("library = {current_version}".to_owned()),
                replace: Some("library = {new_version}".to_owned()),
            })
        );
    }
}
