set positional-arguments

# List development recipes.
default:
    @just --list

# Build the crate.
build *args:
    cargo build "$@"

# Run unprivileged tests, including doctests.
test *args:
    cargo test "$@"

# Lint all targets, treating warnings as errors.
lint *args:
    cargo clippy --all-targets "$@" -- -D warnings

# Format Rust code.
fmt:
    cargo fmt --all

# Check Rust formatting.
fmt-check:
    cargo fmt --all -- --check

# Run privileged mount tests on Linux (requires host sudo).
test-mounts *args:
    bash scripts/tasks/test-mounts.sh "$@"

# Format development tooling.
fmt-tools:
    just --fmt
    shfmt --write --indent 4 scripts/tasks/test-mounts.sh
    nixfmt flake.nix

# Check development tooling formatting and shell lint.
check-tools:
    just --fmt --check
    shfmt --diff --indent 4 scripts/tasks/test-mounts.sh
    nixfmt --check flake.nix
    shellcheck scripts/tasks/test-mounts.sh

# Run unprivileged formatting, lint and test checks.
check: check-tools fmt-check lint test
