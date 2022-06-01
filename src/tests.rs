use crate::core::Version;

pub fn v1() -> Version {
    Version::new()
}

pub fn v2() -> Version {
    Version::with_values(1, 2, 3, None)
}

pub fn v3() -> Version {
    Version::with_values(30, 20, 10, Some("dev.5".to_owned()))
}
