bump-major: PART = major
bump-minor: PART = minor
bump-patch: PART = patch

bump-major bump-minor bump-patch:
	@cargo run -- bump -c .semver.toml --finalize-prerelease
	@cargo run -- bump -c .semver.toml --part $(PART)

.PHONY: bump-major bump-minor bump-patch
