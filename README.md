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

$ semver validate 1.0.0-dev.1+build.1
Version '1.0.0-dev.1+build.1' is valid!
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
Bumped to version: '1.0.1'

$ semver bump -v 1.0.0 -p minor -f semver.toml
Bumped to version: '1.1.0'

$ semver bump -v 1.0.0 -p major -f semver.toml
Bumped to version: '2.0.0'

$ semver bump -v 2.0.0 -p major -f semver.toml
Error: File(NoOp("version '2.0.0' not found in file 'semver.toml'"))
```

## Using a configuration file

The `bump` subcommands accept a configuration file through the `-c` flag. The configuration file is a TOML file where the following informations are stored:
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
$ semver bump -c semver.toml  // equivalent to `semver bump -v 1.0.0 -p minor -f test-1.txt`
```

You can also override the `default_part`:
```
$ semver bump -c semver.toml -p major  // equivalent to `semver bump -v 1.0.0 -p major -f test-1.txt`
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

## Support for prereleases

When using the configuration file, it's possible to support prereleases. It requires to define the JS code with the logic for bumping the `prerelease` part of the version. This is an example configuration:
```
[semver]
current_version = "1.0.0"
default_part = "minor"

  [semver.files]

    [semver.files."test-1.txt"]

  [semver.prerelease]
  bump_script = '''
var PREFIX = "dev.";
function bump(version) {
  var counter = !version.prerelease ? 0 : parseInt(version.prerelease.slice(PREFIX.length));
  return `${PREFIX}${counter + 1}`;
}
'''
```

The `[semver.prerelease]` section has a `bump_script` configuration that contains a `bump` function. The function must be named exactly `bump` and will take `version` as input. `version` is an object with the keys:
- `major`,
- `minor`,
- `patch`,
- `prerelease`.

Given that the current version is `1.0.0` the object that will be passed is:
```
{
  "major": "1",
  "minor": "0",
  "patch": "0",
  "prerelease": null,
}
```

The value returned by the `bump` function will be used for the `prerelease` part.

### Start a new prerelease

Let's say that we're currently at version `1.0.0` and we want to start a new prerelease for the next minor version. According to our example `bump_script` that would be `1.1.0-dev.1`. In order to do so we run the following:
```
$ semver bump -c semver.toml --new-prerelease
Bumped to version: '1.1.0-dev.1'
```

Passing the `-p minor` flag was not required since the `default_part` in the configuration is set to `minor`.

### Bump to next prerelease

After starting the `1.1.0-dev.1` prerelease, let's say we now want to bump to the next prerelease version. According to our example `bump_script` that would be `1.1.0-dev.2`. In order to do so we run the following:
```
$ semver bump -c semver.toml -p prerelease
Bumped to version: '1.1.0-dev.2'
```

### Finalize a prerelease

After two prereleases, we want to finally ship the new final minor version that is `1.1.0`. In order to do so we run the following:
```
$ semver bump -c semver.toml --finalize-prerelease
Bumped to version: '1.1.0
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
