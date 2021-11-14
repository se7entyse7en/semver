use super::{Version, VersionError};
use std::str::FromStr;

#[test]
fn test_version_init() {
    let v1 = Version::new();
    assert_eq!(v1.major, 0);
    assert_eq!(v1.minor, 0);
    assert_eq!(v1.patch, 0);

    let v2 = Version::with_values(1, 2, 3);
    assert_eq!(v2.major, 1);
    assert_eq!(v2.minor, 2);
    assert_eq!(v2.patch, 3);

    let v3 = Version::with_values(30, 20, 10);
    assert_eq!(v3.major, 30);
    assert_eq!(v3.minor, 20);
    assert_eq!(v3.patch, 10);

    let supported_versions = vec![
        ("0.0.0", Version::with_values(0, 0, 0)),
        ("1.2.3", Version::with_values(1, 2, 3)),
        ("30.20.10", Version::with_values(30, 20, 10)),
    ];
    for item in supported_versions {
        assert_eq!(item.0.parse::<Version>(), Version::from_str(item.0));
        match Version::from_str(item.0) {
            Ok(actual) => assert_eq!(actual, item.1),
            Err(_) => panic!("version `{}` should be supported but it's not", item.0),
        }
    }

    let invalid_versions = vec![
        ("00.01.00", VersionError::InvalidVersion),
        ("1.2.3.dev1", VersionError::InvalidVersion),
        ("v1.2.3", VersionError::InvalidVersion),
        ("1", VersionError::InvalidVersion),
    ];
    for item in invalid_versions {
        assert_eq!(item.0.parse::<Version>(), Version::from_str(item.0));
        match Version::from_str(item.0) {
            Ok(actual) => panic!("version `{}` is surprisingly valid: {:?}", item.0, actual),
            Err(err) => assert_eq!(err, item.1(item.0.to_owned())),
        }
    }

    let unsupported_versions = vec![
        ("0.1.0-dev.1", VersionError::UnsupportedVersion),
        ("1.2.3-alpha.5+100", VersionError::UnsupportedVersion),
        ("1.2.3-beta+b.500", VersionError::UnsupportedVersion),
        ("30.20.10+build123", VersionError::UnsupportedVersion),
    ];
    for item in unsupported_versions {
        assert_eq!(item.0.parse::<Version>(), Version::from_str(item.0));
        match Version::from_str(item.0) {
            Ok(actual) => panic!(
                "version `{}` is surprisingly supported: {:?}",
                item.0, actual
            ),
            Err(err) => assert_eq!(err, item.1(item.0.to_owned())),
        }
    }
}

#[test]
fn test_version_bump() {
    let v1 = Version::new();
    assert_eq!(v1.bump_major(), Version::with_values(1, 0, 0));
    assert_eq!(v1.bump_minor(), Version::with_values(0, 1, 0));
    assert_eq!(v1.bump_patch(), Version::with_values(0, 0, 1));

    let v2 = Version::with_values(1, 2, 3);
    assert_eq!(v2.bump_major(), Version::with_values(2, 2, 3));
    assert_eq!(v2.bump_minor(), Version::with_values(1, 3, 3));
    assert_eq!(v2.bump_patch(), Version::with_values(1, 2, 4));
}
