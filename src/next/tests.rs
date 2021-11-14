use super::next;
use crate::core::{Version, VersionError};

#[test]
fn test_next() {
    let valid = vec![
        ("0.0.0", "major", Version::with_values(1, 0, 0)),
        ("0.0.0", "minor", Version::with_values(0, 1, 0)),
        ("0.0.0", "patch", Version::with_values(0, 0, 1)),
        ("1.2.3", "major", Version::with_values(2, 2, 3)),
        ("1.2.3", "minor", Version::with_values(1, 3, 3)),
        ("1.2.3", "patch", Version::with_values(1, 2, 4)),
    ];
    for item in valid {
        match next(item.0, item.1) {
            Ok(actual) => assert_eq!(actual, item.2),
            Err(err) => panic!(
                "unexpected version error when bumping `{}` on version `{}`: {:?}",
                item.1, item.0, err,
            ),
        }
    }

    let invalid_parts = vec![
        ("0.0.0", "a", VersionError::InvalidPart),
        ("0.0.0", "1", VersionError::InvalidPart),
        ("0.0.0", "", VersionError::InvalidPart),
    ];
    for item in invalid_parts {
        match next(item.0, item.1) {
            Ok(_) => panic!("part `{}` is surprisingly valid", item.1),
            Err(err) => assert_eq!(err, item.2(item.1.to_owned())),
        }
    }

    let invalid_versions = vec![
        ("00.01.00", "major", VersionError::InvalidVersion),
        ("1.2.3.dev1", "minor", VersionError::InvalidVersion),
        ("v1.2.3", "patch", VersionError::InvalidVersion),
        ("1", "major", VersionError::InvalidVersion),
    ];
    for item in invalid_versions {
        match next(item.0, item.1) {
            Ok(_) => panic!("version `{}` is surprisingly valid", item.0),
            Err(err) => assert_eq!(err, item.2(item.0.to_owned())),
        }
    }

    let unsupported_versions = vec![
        ("0.1.0-dev.1", "major", VersionError::UnsupportedVersion),
        (
            "1.2.3-alpha.5+100",
            "minor",
            VersionError::UnsupportedVersion,
        ),
        (
            "1.2.3-beta+b.500",
            "patch",
            VersionError::UnsupportedVersion,
        ),
        (
            "30.20.10+build123",
            "major",
            VersionError::UnsupportedVersion,
        ),
    ];
    for item in unsupported_versions {
        match next(item.0, item.1) {
            Ok(_) => panic!("version `{}` is surprisingly supported", item.0),
            Err(err) => assert_eq!(err, item.2(item.0.to_owned())),
        }
    }
}
