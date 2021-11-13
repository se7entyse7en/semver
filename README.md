# semver

CLI to bump and check versions according to semver

## Tagging and publishing

Each push to `master` will make the CI create a development tag as `X.Y.Z-dev.W` and will build the binaries and create a Github Release. Whenever a non-development version needs to be created, then do the following:
1. locally run `make bump-{patch|minor|major}`,
2. open a PR,
3. merge to `master`

The CI will detect the new untagged version and will create the corresponding tag and create the release.

By default, a newly merged PR starting from a non-dev tag, will bump both minor and dev. E.g.:
```
0.1.0 -> 0.2.0-dev.1
```
