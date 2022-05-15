use super::{v1, v2, v3};
use crate::core::{CorePart, ExtensionPart, Version, VersionError};

#[test]
fn test_init() {
    let v1 = v1();
    assert_eq!(v1.major, 0);
    assert_eq!(v1.minor, 0);
    assert_eq!(v1.patch, 0);
    assert_eq!(v1.prerelease, None);

    let v2 = v2();
    assert_eq!(v2.major, 1);
    assert_eq!(v2.minor, 2);
    assert_eq!(v2.patch, 3);
    assert_eq!(v2.prerelease, None);

    let v3 = v3();
    assert_eq!(v3.major, 30);
    assert_eq!(v3.minor, 20);
    assert_eq!(v3.patch, 10);
    assert_eq!(v3.prerelease, Some("dev.5".to_owned()));
}

#[test]
fn test_parse() {
    let test_cases_supported = vec![
        ("0.0.0", Version::with_values(0, 0, 0, None)),
        ("1.2.3", Version::with_values(1, 2, 3, None)),
        ("30.20.10", Version::with_values(30, 20, 10, None)),
        (
            "30.20.10-dev.5",
            Version::with_values(30, 20, 10, Some("dev.5".to_owned())),
        ),
    ];
    for tc in test_cases_supported {
        assert_eq!(tc.0.parse::<Version>().unwrap(), tc.1);
    }

    let invalid_versions = vec!["00.01.00", "1.2.3.dev1", "v1.2.3", "1"];
    for v in invalid_versions {
        matches!(
            v.parse::<Version>().unwrap_err(),
            VersionError::InvalidVersion(_)
        );
    }

    let unsupported_versions = vec!["1.2.3-alpha.5+100", "1.2.3-beta+b.500", "30.20.10+build123"];
    for v in unsupported_versions {
        matches!(
            v.parse::<Version>().unwrap_err(),
            VersionError::UnsupportedVersion(_)
        );
    }
}

#[test]
fn test_to_string() {
    assert_eq!(v1().to_string(), "0.0.0");
    assert_eq!(v2().to_string(), "1.2.3");
    assert_eq!(v3().to_string(), "30.20.10-dev.5");
}

#[test]
fn test_getters() {
    let v1 = v1();
    assert_eq!(v1.get_core_part(&CorePart::Major), 0);
    assert_eq!(v1.get_core_part(&CorePart::Minor), 0);
    assert_eq!(v1.get_core_part(&CorePart::Patch), 0);
    assert_eq!(v1.get_extension_part(&ExtensionPart::Prerelease), None);

    let v2 = v2();
    assert_eq!(v2.get_core_part(&CorePart::Major), 1);
    assert_eq!(v2.get_core_part(&CorePart::Minor), 2);
    assert_eq!(v2.get_core_part(&CorePart::Patch), 3);
    assert_eq!(v2.get_extension_part(&ExtensionPart::Prerelease), None);

    let v3 = v3();
    assert_eq!(v3.get_core_part(&CorePart::Major), 30);
    assert_eq!(v3.get_core_part(&CorePart::Minor), 20);
    assert_eq!(v3.get_core_part(&CorePart::Patch), 10);
    assert_eq!(
        v3.get_extension_part(&ExtensionPart::Prerelease),
        Some("dev.5".to_owned())
    );
}
