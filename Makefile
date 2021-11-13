bump-major: PART = major
bump-minor: PART = minor
bump-patch: PART = patch
bump-dev: PART = dev

bump-major bump-minor bump-patch bump-dev:
	@python bump.py --target $(PART)

tag:
	@./tag.sh $(TAG)

.PHONY: bump-major bump-minor bump-patch bump-dev tag act
