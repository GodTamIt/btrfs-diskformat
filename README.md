# BTRFS Disk Format

[![Crates.io](https://img.shields.io/crates/v/btrfs-diskformat.svg)](https://crates.io/crates/btrfs-diskformat)
[![Crates.io](https://img.shields.io/badge/license-BSD%202--Clause-blue)](https://crates.io/crates/btrfs-diskformat)

Clean-room implementation of the [btrfs] disk format in Rust.

[Documentation](https://docs.rs/btrfs-diskformat/)

## License

`btrfs-diskformat` is distributed under the terms of the BSD 2-Clause license.

See the [LICENSE-BSD](LICENSE-BSD) file in this repository for more information.

## Cargo Features

- `alloc`: Enables allocation and the `alloc` feature in `zerocopy`.
- `std`: By default, the crate is `no_std`. This enables `std` features from
  the `zerocopy` and `strum` dependencies.

## Development

Enter the pinned tool shell with `nix develop` (or `direnv allow` with direnv's
Nix integration enabled), then use:

```sh
just --list       # or just
just check        # tooling checks, Rust formatting, lint, and unprivileged tests
just build
just test         # includes doctests; privileged tests remain ignored
just fmt
just test-mounts  # Linux only; requires host sudo
```

Without Nix, install Rust and the tools used by the recipes and run the same
commands. `just fmt-tools` formats the justfile, shell script, and Nix flake.
The shell supplies userspace tools, not a kernel or privilege configuration:
mount tests still require Linux >= 6.7 with loop-device and Btrfs support and
host-configured passwordless sudo. The privileged run trusts the checkout, built
test executable, and every tool on the preserved `PATH` as root. A private mount
namespace is not a VM.

## Integration tests

Privileged mount tests use only temporary sparse file-backed disks, never real disks.
They are ignored by normal `cargo test`. Run them with:

```sh
bash scripts/tasks/test-mounts.sh
```

Requires Linux kernel >= 6.7, `btrfs-progs` >= 6.6.2 (6.6.3 recommended),
util-linux tools, Python 3, Rust, and passwordless sudo. Run the script as an
unprivileged user: it builds without privileges, then runs tests serially as root
in a private mount namespace.

Coverage includes checksums, compression, metadata UUID, free-space and block-group
trees, quotas, mixed block groups, and multi-device RAID. Experimental RAID-stripe
and remap features and physical zoned devices are not exercised: host kernels and
tools lack finalized support.

## Contributing

Because this codebase is developed without knowledge of the Linux btrfs source code and is released under a more permissive license(s) than GPLv2, development is heavily dependent on information released on the [btrfs wiki] and reverse engineering the effects of operations made by `btrfs-progs` and other utilities. As a result, contributions to this codebase must strictly follow the same siloed approach.

[btrfs]: https://btrfs.wiki.kernel.org/index.php/Main_Page
[btrfs wiki]: https://btrfs.wiki.kernel.org/index.php/Main_Page
