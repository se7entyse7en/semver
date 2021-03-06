# HISTORY

## Unreleased

## [v0.2.0 - 2022-06-19](https://github.com/se7entyse7en/semver/compare/v0.1.0...v0.2.0)

### Added

- Added support for prereleases (`bump --new-prerelease/--finalize-prerelease`) with the ability to specify in the configuration the JS code for bumping the prerelease part
- The config file keeps track also of the latest stable (non-prerelease) version
- Added support for `search` and `replace` in config
- Added support for `stable_only` in config

### Changed

- The `validate` subcommand supports prereleases
- The `--part` cmd arg is now checked against the set of possible values instead of being an arbitrary string
- [internal] Dogfood `semver` :tada:

### Removed

- Removed `next` subcommand
- Removed `hello` subcommand

### Fixed

- Fixed logic when bumping by properly resetting the parts "on the right" (e.g. major bump for 1.2.3 is 2.0.0 and not 2.2.3)

## [v0.1.0 - 2022-04-16](https://github.com/se7entyse7en/semver/compare/v0.0.0...v0.1.0)

### Added

- Added `hello` subcommand
- Added `validate` subcommand that checks whether the provided version complies to semver
- Added `next` subcommand that accepts a version and computes the next one according to the part that has to be bumped either as cmd args or from config file
- Added `bump` subcommand that accepts a version and replaces it in a file with the next one according to the part the has to be bumped either as cmd args or from config file

## [v0.0.0 - 2021-11-13](https://github.com/se7entyse7en/semver/compare/486f8cd34136f830e21c15ff179a74a251165fd9...v0.0.0)

- Project inception! :tada:
