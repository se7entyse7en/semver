mod test_version;
mod test_version_manager;

use crate::core::Version;

fn v1() -> Version {
    Version::new()
}

fn v2() -> Version {
    Version::with_values(1, 2, 3, None)
}

fn v3() -> Version {
    Version::with_values(30, 20, 10, Some("dev.5".to_owned()))
}
