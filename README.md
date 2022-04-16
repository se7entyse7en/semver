# semver

CLI to bump and check versions according to semver

## Quickstart

### Installation

`semver` can be downloaded from the [releases page](https://github.com/se7entyse7en/semver/releases).

### How to validate if a version is semver-compliant

```
semver validate <VERSION>
```

Examples:
```
$ semver validate abc
Version 'abc' is not valid!

$ semver validate v1.0.0
Version 'v1.0.0' is not valid!

$ semver validate 1.0.0
Version '1.0.0' is valid!
```

### How to compute the next version

```
semver next -v <VERSION> -p <PART>
```

Examples:
```
$ semver next -v 1.0.0 -p patch
Next version: '1.0.1'

$ semver next -v 1.0.0 -p minor
Next version: '1.1.0'

$ semver next -v 1.0.0 -p major
Next version: '2.0.0
```

### How to bump a file to the next version

```
semver bump -v <VERSION> -p <PART> -f <FILE>
```

We assume to have a `semver.toml` file whose content is:
```
current_version = "1.0.0"
```

Examples:
```
$ semver bump -v 1.0.0 -p patch -f semver.toml
Bumpbed to version: '1.0.1'

$ semver bump -v 1.0.0 -p minor -f semver.toml
Bumpbed to version: '1.1.0'

$ semver bump -v 1.0.0 -p major -f semver.toml
Bumpbed to version: '2.0.0'

$ semver bump -v 2.0.0 -p major -f semver.toml
Error: NoOp("version '2.0.0' not found in file 'semver.toml'")
```

## Using a configuration file

Both the `next` and the `bump` subcommands accept a configuration file through the `-c` flag. The configuration file is a TOML file where the following informations are stored:
- the current version,
- the default `part` to use if not specified,
- the list of files that has to be bumped when running the `bump` subcommand.

Let's assume to have a configuration file `semver.toml` whose content is:
```
[semver]
current_version = "1.0.0"
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]
```

We can simply run the followings:
```
$ semver next -c semver.toml  // equivalent to `semver next -v 1.0.0 -p minor`
$ semver bump -c semver.toml  // equivalent to `semver bump -v 1.0.0 -p minor -f test-1.txt`
```

You can also override the `default_part`:
```
$ semver next -c semver.toml -p patch  // equivalent to `semver next -v 1.0.0 -p patch`
$ semver bump -c semver.toml -p majod  // equivalent to `semver bump -v 1.0.0 -p major -f test-1.txt`
```

When the configuration file is provided, the `bump` command automatically bumps also the configuration file itself.

Through the configuration file you can specify multiple files to bump which is currently not possible throught cmd args:
```
[semver]
current_version = "1.0.0"
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]

    [semver.files."test-2.txt"]

    [semver.files."test-3.txt"]
```

## Development

### Tagging and publishing

Each push to `master` will make the CI create a development tag as `X.Y.Z-dev.W` and will build the binaries and create a Github Release. Whenever a non-development version needs to be created, then do the following:
1. locally run `make bump-{patch|minor|major}`,
2. open a PR,
3. merge to `master`

The CI will detect the new untagged version and will create the corresponding tag and create the release.

By default, a newly merged PR starting from a non-dev tag, will bump both minor and dev. E.g.:
```
0.1.0 -> 0.2.0-dev.1
```
