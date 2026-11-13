#!/usr/bin/env bash
set -euo pipefail

if [[ $(uname -s) != Linux ]]; then
    printf 'Mount integration tests require Linux.\n' >&2
    exit 1
fi

if ((EUID == 0)); then
    printf 'Run as an unprivileged user; only the test executable is elevated via sudo.\n' >&2
    exit 1
fi

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.."

for tool in cargo rustc unshare mount umount losetup mkfs.btrfs btrfs btrfstune id sudo env; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'Required tool not found: %s\n' "$tool" >&2
        exit 1
    fi
done

if [[ ${1-} == --run ]]; then
    shift
    if [[ -z ${1-} ]]; then
        printf 'Missing test executable.\n' >&2
        exit 1
    fi
    # Preserve dev-shell tools even when sudo resets PATH.
    exec sudo -- "$(command -v env)" "PATH=$PATH" unshare --mount --propagation private -- "$@"
fi

# Exercise the host kernel and override any configured target runner.
host=$(rustc --print host-tuple)
runner="CARGO_TARGET_${host^^}_RUNNER"
export "${runner//-/_}=bash scripts/tasks/test-mounts.sh --run"
exec cargo test --target "$host" --test mount -- --ignored --test-threads=1 "$@"
