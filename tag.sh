set -e
if [ -z "$1" ]
then
    version=$(bumpversion --dry-run --list dev | grep current_version | cut -c 17-)
else
    version="$1"
fi
git tag -a "v${version}" -m "Version ${version}";
set +e
