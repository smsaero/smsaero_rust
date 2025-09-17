PACKAGE_NAME = smsaero
CARGO = cargo
GIT = git

.PHONY: help check test build clean publish dry-run version bump-patch bump-minor bump-major

help:
	@echo "SmsAero Rust API - Available commands:"
	@echo ""
	@echo "Basic commands:"
	@echo "  help          - Show this help"
	@echo "  check         - Check code without compilation"
	@echo "  test          - Run tests"
	@echo "  build         - Build project"
	@echo "  clean         - Clean build artifacts"
	@echo ""
	@echo "Publishing:"
	@echo "  dry-run       - Test publish (without actual upload)"
	@echo "  publish       - Publish release to crates.io"
	@echo ""
	@echo "Version management:"
	@echo "  bump-patch    - Increment patch version (1.0.0 -> 1.0.1)"
	@echo "  bump-minor    - Increment minor version (1.0.0 -> 1.1.0)"
	@echo "  bump-major    - Increment major version (1.0.0 -> 2.0.0)"
	@echo "  version       - Show current version"
	@echo ""
	@echo "Full release cycle:"
	@echo "  release-patch - Full patch release cycle"
	@echo "  release-minor - Full minor release cycle"
	@echo "  release-major - Full major release cycle"

check:
	@echo "Checking code..."
	$(CARGO) check
	$(CARGO) clippy -- -D warnings -A clippy::too_many_arguments
	$(CARGO) fmt --check

test:
	@echo "Running tests..."
	$(CARGO) test

build:
	@echo "Building project..."
	$(CARGO) build --release

clean:
	@echo "Cleaning artifacts..."
	$(CARGO) clean

version:
	@echo "Current version:"
	@grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'

bump-patch:
	@echo "Incrementing patch version..."
	$(CARGO) set-version --bump patch
	@echo "Patch version incremented"

bump-minor:
	@echo "Incrementing minor version..."
	$(CARGO) set-version --bump minor
	@echo "Minor version incremented"

bump-major:
	@echo "Incrementing major version..."
	$(CARGO) set-version --bump major
	@echo "Major version incremented"

dry-run:
	@echo "Test publishing..."
	$(CARGO) publish --dry-run

publish:
	@echo "Publishing to crates.io..."
	$(CARGO) publish

release-patch: check test bump-patch
	@echo "Creating patch release..."
	@$(MAKE) version
	@echo "Creating git tag..."
	@VERSION=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	$(GIT) add Cargo.toml Cargo.lock; \
	$(GIT) commit -m "Bump version to v$$VERSION"; \
	$(GIT) tag -a "v$$VERSION" -m "Release v$$VERSION"; \
	echo "Patch release v$$VERSION ready!"; \
	echo "To publish run: make publish"

release-minor: check test bump-minor
	@echo "Creating minor release..."
	@$(MAKE) version
	@echo "Creating git tag..."
	@VERSION=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	$(GIT) add Cargo.toml Cargo.lock; \
	$(GIT) commit -m "Bump version to v$$VERSION"; \
	$(GIT) tag -a "v$$VERSION" -m "Release v$$VERSION"; \
	echo "Minor release v$$VERSION ready!"; \
	echo "To publish run: make publish"

release-major: check test bump-major
	@echo "Creating major release..."
	@$(MAKE) version
	@echo "Creating git tag..."
	@VERSION=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	$(GIT) add Cargo.toml Cargo.lock; \
	$(GIT) commit -m "Bump version to v$$VERSION"; \
	$(GIT) tag -a "v$$VERSION" -m "Release v$$VERSION"; \
	echo "Major release v$$VERSION ready!"; \
	echo "To publish run: make publish"

pre-publish:
	@echo "Pre-publish checks..."
	@echo "1. Checking git status..."
	@if [ -n "$$($(GIT) status --porcelain)" ]; then \
		echo "Error: Uncommitted changes in git"; \
		exit 1; \
	fi
	@echo "2. Checking code..."
	@$(MAKE) check
	@echo "3. Running tests..."
	@$(MAKE) test
	@echo "4. Test publishing..."
	@$(MAKE) dry-run
	@echo "All checks passed! Ready to publish."

publish-safe: pre-publish publish
	@echo "Publication completed successfully!"

info:
	@echo "Package information:"
	@echo "Name: $(PACKAGE_NAME)"
	@echo "Version: $$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"
	@echo "Authors: $$(grep '^authors = ' Cargo.toml | sed 's/authors = \[\(.*\)\]/\1/')"
	@echo "License: $$(grep '^license = ' Cargo.toml | sed 's/license = "\(.*\)"/\1/')"
	@echo "Repository: $$(grep '^repository = ' Cargo.toml | sed 's/repository = "\(.*\)"/\1/')"
