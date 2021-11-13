import argparse
import configparser
import re
import subprocess

parser = argparse.ArgumentParser(description="Bump version")
parser.add_argument(
    "--target",
    choices=["major", "minor", "patch", "dev"],
    default="dev",
    help="The target of the version bump",
)

args = parser.parse_args()


def run_cmd(cmd):
    return subprocess.run(cmd, check=True, capture_output=True)


def read_bumpversion_cfg():
    config = configparser.ConfigParser()
    config.read(".bumpversion.cfg")
    return config


def fix_history():
    with open("HISTORY.md", "r") as fin:
        content = fin.read()

    latest_non_dev_version = find_latest_non_dev_version()

    content = re.sub(r"^\-$", "", content, flags=re.MULTILINE)
    content = re.sub(r"^\- ##", "##", content, flags=re.MULTILINE)
    content = content.replace("vXXX", f"v{latest_non_dev_version}")

    with open("HISTORY.md", "w") as fout:
        fout.write(content)


def find_latest_non_dev_version():
    out = subprocess.run(
        "git tag --sort=-taggerdate | grep -v dev | head -n 1",
        check=True,
        capture_output=True,
        shell=True,
    )
    return out.stdout.decode().strip()[1:]


def main():
    config = read_bumpversion_cfg()
    bump_version_conf = dict(config["bumpversion"])
    current_version = bump_version_conf["current_version"]
    version_regex = bump_version_conf["parse"]
    files = [
        s[len("bumpversion:file:") :]
        for s in config.sections()
        if s.startswith("bumpversion:file:")
    ]
    match = re.match(version_regex, current_version).groupdict()
    if args.target == "dev":
        files.remove("HISTORY.md")
        if match["dev"] is None:
            run_cmd(["bumpversion", "--no-configured-files", "minor",] + files)
            run_cmd(
                ["bumpversion", "--no-configured-files", "--allow-dirty", "dev"]
                + files,
            )
        else:
            run_cmd(["bumpversion", "--no-configured-files", "dev",] + files)
    else:
        if match["dev"] is not None:
            if args.target == "major":
                new_version = (
                    f"{match['major'] + 1}.{match['minor'] - 1}.{match['patch']}"
                )
            elif args.target == "minor":
                new_version = f"{match['major']}.{match['minor']}.{match['patch']}"
            else:
                raise ValueError(
                    "Cannot bump from `dev` to `patch`, rebase on top ",
                    "of the latest non-dev release",
                )

            cmd_args = ["--new-version", new_version]
        else:
            cmd_args = []

        cmd_args.append(args.target)
        run_cmd(["bumpversion"] + cmd_args)
        fix_history()

    # Cargo check will update Cargo.lock version. Adding Cargo.lock in the list of
    # files in .bumpverions.cfg is not reliable as it may match the same version
    # of some dependency.
    run_cmd(["cargo", "check"])
    config = read_bumpversion_cfg()
    new_version = dict(config["bumpversion"])["current_version"]
    run_cmd(
        [
            "git",
            "commit",
            "-a",
            "-s",
            "-m",
            f"Bump version: {current_version} → {new_version}",
        ],
    )


if __name__ == "__main__":
    main()
