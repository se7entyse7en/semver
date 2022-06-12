use super::{replace_content, Context};
use chrono::prelude::*;

#[test]
fn test_replace_content() {
    let content = r#"
whatever = 3.0.0
semver = 1.0.0
something = 5.0.0
"#;
    let search = r#"semver = {current_version}"#;
    let replace = r#"semver = {new_version}"#;
    let context =
        Context::with_versions("1.0.0".to_owned(), "2.0.0".to_owned(), "1.0.0".to_owned());

    assert_eq!(
        replace_content(content, search, replace, &context).unwrap(),
        r#"
whatever = 3.0.0
semver = 2.0.0
something = 5.0.0
"#
    );
}

#[test]
fn test_replace_content_with_date() {
    let content = r#"
# HISTORY

## Unreleased

- Some changes

## v1.0.0 - 2022-01-01

- Some changes
"#;
    let search = r#"
## Unreleased
"#;
    let replace = r#"
## Unreleased

## v{new_version} - {local_today_ymd} (diff: v{last_stable_version}..v{new_version})
"#;
    let context = Context::with_versions_and_now(
        "1.0.0".to_owned(),
        "2.0.0".to_owned(),
        "1.0.0".to_owned(),
        Utc.ymd(2022, 6, 10),
        Local.ymd(2022, 6, 10),
    );

    assert_eq!(
        replace_content(content, search, replace, &context).unwrap(),
        r#"
# HISTORY

## Unreleased

## v2.0.0 - 2022-06-10 (diff: v1.0.0..v2.0.0)

- Some changes

## v1.0.0 - 2022-01-01

- Some changes
"#
    );
}
