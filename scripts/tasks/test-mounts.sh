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

for tool in cargo python3 unshare mount umount losetup mkfs.btrfs btrfs btrfstune id sudo env; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'Required tool not found: %s\n' "$tool" >&2
        exit 1
    fi
done

artifacts=$(mktemp)
trap 'rm -f -- "$artifacts"' EXIT
cargo test --test mount --no-run --message-format=json >"$artifacts"
binary=$(
    python3 - "$artifacts" <<'PY'
import json
import sys

with open(sys.argv[1]) as artifacts:
    binaries = [
        artifact["executable"]
        for artifact in map(json.loads, artifacts)
        if artifact.get("reason") == "compiler-artifact"
        and artifact["target"]["name"] == "mount"
        and artifact["profile"]["test"]
        and artifact.get("executable") is not None
    ]
assert len(binaries) == 1, f"Expected one mount test executable, found {len(binaries)}"
print(binaries[0])
PY
)

# Preserve dev-shell tools even when sudo resets PATH.
sudo -- "$(command -v env)" "PATH=$PATH" unshare --mount --propagation private -- "$binary" --ignored --test-threads=1 "$@"
