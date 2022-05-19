use super::{validate, validate_part};
use crate::core::{CorePart, ExtensionPart, Part};

// Ref: https://regex101.com/r/Ly7O1x/3/
#[test]
fn test_correct_versions() {
    let versions = vec![
        "0.0.4",
        "1.2.3",
        "10.20.30",
        "1.1.2-prerelease+meta",
        "1.1.2+meta",
        "1.1.2+meta-valid",
        "1.0.0-alpha",
        "1.0.0-beta",
        "1.0.0-alpha.beta",
        "1.0.0-alpha.beta.1",
        "1.0.0-alpha.1",
        "1.0.0-alpha0.valid",
        "1.0.0-alpha.0valid",
        "1.0.0-alpha-a.b-c-somethinglong+build.1-aef.1-its-okay",
        "1.0.0-rc.1+build.1",
        "2.0.0-rc.1+build.123",
        "1.2.3-beta",
        "10.2.3-DEV-SNAPSHOT",
        "1.2.3-SNAPSHOT-123",
        "1.0.0",
        "2.0.0",
        "1.1.7",
        "2.0.0+build.1848",
        "2.0.1-alpha.1227",
        "1.0.0-alpha+beta",
        "1.2.3----RC-SNAPSHOT.12.9.1--.12+788",
        "1.2.3----R-S.12.9.1--.12+meta",
        "1.2.3----RC-SNAPSHOT.12.9.1--.12",
        "1.0.0+0.build.1-rc.10000aaa-kk-0.1",
        "99999999999999999999999.999999999999999999.99999999999999999",
        "1.0.0-0A.is.legal",
    ];
    for version in versions {
        assert!(validate(version));
        assert!(validate_part(version, None));
    }
}

// Ref: https://regex101.com/r/Ly7O1x/3/
#[test]
fn test_wrong_versions() {
    let wrong_versions = vec![
        "1",
        "1.2",
        "1.2.3-0123",
        "1.2.3-0123.0123",
        "1.1.2+.123",
        "+invalid",
        "-invalid",
        "-invalid+invalid",
        "-invalid.01",
        "alpha",
        "alpha.beta",
        "alpha.beta.1",
        "alpha.1",
        "alpha+beta",
        "alpha_beta",
        "alpha.",
        "alpha..",
        "beta",
        "1.0.0-alpha_beta",
        "-alpha.",
        "1.0.0-alpha..",
        "1.0.0-alpha..1",
        "1.0.0-alpha...1",
        "1.0.0-alpha....1",
        "1.0.0-alpha.....1",
        "1.0.0-alpha......1",
        "1.0.0-alpha.......1",
        "01.1.1",
        "1.01.1",
        "1.1.01",
        "1.2",
        "1.2.3.DEV",
        "1.2-SNAPSHOT",
        "1.2.31.2.3----RC-SNAPSHOT.12.09.1--..12+788",
        "1.2-RC-SNAPSHOT",
        "-1.0.3-gamma+b7718",
        "+justmeta",
        "9.8.7+meta+meta",
        "9.8.7-whatever+meta+meta",
        "99999999999999999999999.999999999999999999.99999999999999999----RC-SNAPSHOT.12.09.1--------------------------------..12",
    ];
    for version in wrong_versions {
        assert!(!validate(version));
        assert!(!validate_part(version, None));
    }
}

#[test]
fn test_correct_parts() {
    let test_cases = vec![
        ("0", Part::Core(CorePart::Major)),
        ("10", Part::Core(CorePart::Major)),
        ("999999", Part::Core(CorePart::Major)),
        ("0", Part::Core(CorePart::Minor)),
        ("10", Part::Core(CorePart::Minor)),
        ("999999", Part::Core(CorePart::Minor)),
        ("0", Part::Core(CorePart::Patch)),
        ("10", Part::Core(CorePart::Patch)),
        ("999999", Part::Core(CorePart::Patch)),
        ("0", Part::Core(CorePart::Patch)),
        ("10", Part::Extension(ExtensionPart::Prerelease)),
        ("999999", Part::Extension(ExtensionPart::Prerelease)),
        ("alpha", Part::Extension(ExtensionPart::Prerelease)),
        ("beta", Part::Extension(ExtensionPart::Prerelease)),
        ("rc", Part::Extension(ExtensionPart::Prerelease)),
        ("dev.1", Part::Extension(ExtensionPart::Prerelease)),
        (
            "---whatever-abc-def.1.ghi---",
            Part::Extension(ExtensionPart::Prerelease),
        ),
    ];
    for tc in test_cases {
        assert!(validate_part(tc.0, Some(&tc.1)));
    }
}

#[test]
fn test_wrong_parts() {
    let test_cases = vec![
        ("a", Part::Core(CorePart::Major)),
        ("-", Part::Core(CorePart::Major)),
        (".", Part::Core(CorePart::Major)),
        ("_", Part::Core(CorePart::Major)),
        ("a", Part::Core(CorePart::Minor)),
        ("-", Part::Core(CorePart::Minor)),
        (".", Part::Core(CorePart::Minor)),
        ("_", Part::Core(CorePart::Minor)),
        ("a", Part::Core(CorePart::Patch)),
        (".", Part::Core(CorePart::Patch)),
        ("-", Part::Core(CorePart::Patch)),
        ("_", Part::Core(CorePart::Patch)),
        ("alpha..", Part::Extension(ExtensionPart::Prerelease)),
        ("beta..1", Part::Extension(ExtensionPart::Prerelease)),
        ("rc_1", Part::Extension(ExtensionPart::Prerelease)),
        ("_alpha_", Part::Extension(ExtensionPart::Prerelease)),
        (".dev.", Part::Extension(ExtensionPart::Prerelease)),
    ];
    for tc in test_cases {
        assert!(!validate_part(tc.0, Some(&tc.1)));
    }
}
