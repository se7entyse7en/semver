use crate::cmd::helpers;
use crate::{config, core};
use clap::Args;
use std::collections::HashMap;

#[derive(Args)]
pub struct BumpArgs {
    /// Current version from which to compute the next one
    #[clap(short = 'v', long, display_order = 1)]
    current_version: Option<String>,

    /// Which part of the version to bump
    ///
    /// If the current version is a "prerelease", then this is ignored and a "prerelease" bump is assumed.
    #[clap(short, long, display_order = 2)]
    part: Option<core::Part>,

    /// Starts a new prerelease for the provided part or increase the current prerelease.
    ///
    /// This is incompatible with `--finalize-prerelease`.
    #[clap(long, display_order = 3)]
    new_prerelease: bool,

    /// Finalize the current prerelease
    ///
    /// The argument `-p, --part <PART>` is ignored. This is incompatible with `---prerelease`.
    #[clap(long, display_order = 4)]
    finalize_prerelease: bool,

    // TODO: Handle multiple files
    /// File containing the version to bump
    #[clap(short, long, display_order = 5)]
    file: Option<String>,

    /// Path of the configuration file
    #[clap(short, long, display_order = 6)]
    config: Option<String>,
}

pub struct FinalizedBumpArgs {
    pub current_version: String,
    pub last_stable_version: Option<String>,
    pub part: core::Part,
    pub new_prerelease: bool,
    pub finalize_prerelease: bool,
    pub bump_prerelease_func: Option<Box<dyn core::ExtensionBumpFunc>>,
    pub files: HashMap<String, config::FileConfig>,
    pub original_config: Option<config::Config>,
}

impl helpers::FinalizeArgs for BumpArgs {
    type FinalizedArgs = FinalizedBumpArgs;

    fn get_config(&self) -> Option<String> {
        self.config.to_owned()
    }

    fn get_required_args(&self) -> Vec<String> {
        vec![
            "current_version".to_owned(),
            "part".to_owned(),
            "file".to_owned(),
        ]
    }

    fn finalize_from_config(&self, config: config::Config) -> Self::FinalizedArgs {
        let original_config = config.clone();
        FinalizedBumpArgs {
            current_version: config.current_version,
            last_stable_version: config.last_stable_version,
            part: self.part.to_owned().unwrap_or(config.default_part),
            new_prerelease: self.new_prerelease,
            finalize_prerelease: self.finalize_prerelease,
            files: config.files,
            bump_prerelease_func: config
                .bump_prerelease_func
                .map(|code| helpers::build_bump_func(code).unwrap()),
            original_config: Some(original_config),
        }
    }

    fn finalize_from_self(&self) -> Option<Self::FinalizedArgs> {
        match (
            self.current_version.as_ref(),
            self.part.as_ref(),
            self.file.as_ref(),
        ) {
            (Some(current_version), Some(part), Some(file)) => Some(FinalizedBumpArgs {
                current_version: current_version.to_owned(),
                last_stable_version: None,
                part: part.to_owned(),
                new_prerelease: self.new_prerelease,
                finalize_prerelease: self.finalize_prerelease,
                files: HashMap::from([(file.to_owned(), config::FileConfig::new())]),
                bump_prerelease_func: None,
                original_config: None,
            }),
            _ => None,
        }
    }
}
