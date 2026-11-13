#![cfg(target_os = "linux")]

mod support;

use std::{
    fs,
    io::Write,
    os::unix::fs::{MetadataExt, symlink},
    process::Command,
};

use btrfs_diskformat::{
    AllocationType, ChecksumType, Chunk, CompatRoFlags, IncompatFlags, Key, KeyType,
    ReplicationPolicy, Stripe, SuperBlock, constants::MAGIC,
};
use support::DiskSet;
use zerocopy::FromBytes;

fn roundtrip(disks: &mut DiskSet, options: &str) {
    let data: Vec<u8> = (0..128 * 1024).map(|i| b'a' + (i % 23) as u8).collect();
    disks.mount(options);
    let directory = disks.mountpoint().join("roundtrip");
    fs::create_dir(&directory).unwrap();
    let original = directory.join("original");
    let mut file = fs::File::create(&original).unwrap();
    file.write_all(&data).unwrap();
    file.sync_all().unwrap();
    drop(file);
    fs::hard_link(&original, directory.join("hardlink")).unwrap();
    symlink("original", directory.join("symlink")).unwrap();
    disks.unmount();

    disks.mount("ro");
    for name in ["original", "hardlink", "symlink"] {
        assert_eq!(fs::read(disks.mountpoint().join("roundtrip").join(name)).unwrap(), data);
    }
    let original = fs::metadata(disks.mountpoint().join("roundtrip/original")).unwrap();
    let hardlink = fs::metadata(disks.mountpoint().join("roundtrip/hardlink")).unwrap();
    assert_eq!(original.ino(), hardlink.ino());
    assert_eq!(original.nlink(), 2);
    assert_eq!(
        fs::read_link(disks.mountpoint().join("roundtrip/symlink")).unwrap(),
        std::path::Path::new("original")
    );
    disks.unmount();
}

fn check_superblocks(superblocks: &[SuperBlock], devices: usize, checksum: ChecksumType) {
    assert_eq!(superblocks.len(), devices);
    let fsid = superblocks[0].fsid;
    for sb in superblocks {
        assert_eq!(sb.magic.get(), MAGIC);
        assert_eq!(sb.num_devices.get(), devices as u64);
        assert_eq!(sb.fsid, fsid);
        let csum_type = sb.csum_type;
        assert_eq!(csum_type, checksum);
    }
}

fn check_system_profile(sb: &SuperBlock, profile: ReplicationPolicy) {
    let mut bytes = &sb.sys_chunk_array[..sb.sys_chunk_array_size.get() as usize];
    assert!(!bytes.is_empty());
    while !bytes.is_empty() {
        let (key, remaining) = Key::read_from_prefix(bytes).unwrap();
        assert_eq!(key.key_type, KeyType::ChunkItem as u8);
        let (chunk, mut remaining) = Chunk::read_from_prefix(remaining).unwrap();
        assert_ne!(chunk.chunk_type.get() & AllocationType::SYSTEM.bits(), 0);
        assert_eq!(chunk.chunk_type.get() & ReplicationPolicy::all().bits(), profile.bits());
        assert!(chunk.num_stripes.get() > 0);
        for _ in 0..chunk.num_stripes.get() {
            let (stripe, rest) = Stripe::read_from_prefix(remaining).unwrap();
            assert!((1..=sb.num_devices.get()).contains(&stripe.devid.get()));
            remaining = rest;
        }
        bytes = remaining;
    }
}

fn scenario(
    devices: usize,
    args: &[&str],
    options: &str,
    checksum: ChecksumType,
    inspect: impl Fn(&SuperBlock),
) {
    let mut disks = DiskSet::format(devices, args);
    roundtrip(&mut disks, options);
    let superblocks = disks.superblocks();
    check_superblocks(&superblocks, devices, checksum);
    for sb in &superblocks {
        inspect(sb);
    }
    disks.finish();
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn single_device_default() {
    scenario(1, &[], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::DUP);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn checksum_crc32c() {
    scenario(1, &["--checksum", "crc32c"], "", ChecksumType::CRC32C, |_| {});
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn checksum_xxhash() {
    scenario(1, &["--checksum", "xxhash"], "", ChecksumType::XXHASH64, |_| {});
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn checksum_sha256() {
    scenario(1, &["--checksum", "sha256"], "", ChecksumType::SHA256, |_| {});
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn checksum_blake2b() {
    scenario(1, &["--checksum", "blake2b"], "", ChecksumType::BLAKE2b, |_| {});
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn mixed_data_metadata() {
    scenario(1, &["-M", "--nodesize", "4096"], "", ChecksumType::CRC32C, |sb| {
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::MIXED_GROUPS.bits(), 0);
        assert_eq!(sb.nodesize.get(), 4096);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn free_space_tree() {
    scenario(1, &["-O", "^block-group-tree,free-space-tree"], "", ChecksumType::CRC32C, |sb| {
        let expected = CompatRoFlags::FREE_SPACE_TREE | CompatRoFlags::FREE_SPACE_TREE_VALID;
        assert_eq!(sb.compat_ro_flags.get() & expected.bits(), expected.bits());
        assert_eq!(sb.compat_ro_flags.get() & CompatRoFlags::BLOCK_GROUP_TREE.bits(), 0);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn block_group_tree() {
    scenario(1, &["-O", "block-group-tree"], "", ChecksumType::CRC32C, |sb| {
        let expected = CompatRoFlags::FREE_SPACE_TREE
            | CompatRoFlags::FREE_SPACE_TREE_VALID
            | CompatRoFlags::BLOCK_GROUP_TREE;
        assert_eq!(sb.compat_ro_flags.get() & expected.bits(), expected.bits());
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::NO_HOLES.bits(), 0);
    });
}

fn quota_scenario(feature: &str, simple: bool) {
    let mut disks = DiskSet::format(1, &["-O", feature]);
    disks.mount("");
    let output = Command::new("btrfs")
        .args(["qgroup", "show"])
        .arg(disks.mountpoint())
        .output()
        .expect("execute btrfs qgroup show on mounted quota filesystem");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "btrfs qgroup show {:?}: {}; stdout: {stdout}; stderr: {stderr}",
        disks.mountpoint(),
        output.status
    );
    assert!(
        stdout.lines().any(|line| line.split_whitespace().next() == Some("0/5")),
        "btrfs qgroup show {:?}: missing default qgroup 0/5; stdout: {stdout}; stderr: {stderr}",
        disks.mountpoint()
    );
    disks.unmount();
    roundtrip(&mut disks, "");
    let superblocks = disks.superblocks();
    check_superblocks(&superblocks, 1, ChecksumType::CRC32C);
    for sb in &superblocks {
        assert_eq!(sb.incompat_flags.get() & IncompatFlags::SIMPLE_QUOTA.bits() != 0, simple);
    }
    disks.finish();
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn quota() {
    quota_scenario("quota", false);
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn simplequota() {
    quota_scenario("squota", true);
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn compression_lzo() {
    scenario(1, &[], "compress-force=lzo", ChecksumType::CRC32C, |sb| {
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::COMPRESS_LZO.bits(), 0);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn compression_zstd() {
    scenario(1, &[], "compress-force=zstd", ChecksumType::CRC32C, |sb| {
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::COMPRESS_ZSTD.bits(), 0);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn metadata_uuid() {
    let mut disks = DiskSet::format(1, &[]);
    let original = disks.superblocks()[0].fsid;
    disks.tune(&["-f", "-m"]);
    roundtrip(&mut disks, "");
    let superblocks = disks.superblocks();
    check_superblocks(&superblocks, 1, ChecksumType::CRC32C);
    for sb in &superblocks {
        assert_ne!(sb.fsid, original);
        assert_eq!(sb.metadata_uuid, original);
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::METADATA_UUID.bits(), 0);
    }
    disks.finish();
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn raid0() {
    scenario(2, &["--data", "raid0", "--metadata", "raid1"], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::RAID1)
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn raid1() {
    scenario(2, &["--data", "raid1", "--metadata", "raid1"], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::RAID1)
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn raid10() {
    scenario(4, &["--data", "raid10", "--metadata", "raid10"], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::RAID10)
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn raid1c3() {
    scenario(3, &["--data", "raid1c3", "--metadata", "raid1c3"], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::RAID1C3);
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::RAID1C34.bits(), 0);
    });
}

#[test]
#[ignore = "requires root, loop devices, Btrfs kernel support, and btrfs-progs 6.6.2 or newer"]
fn raid1c4() {
    scenario(4, &["--data", "raid1c4", "--metadata", "raid1c4"], "", ChecksumType::CRC32C, |sb| {
        check_system_profile(sb, ReplicationPolicy::RAID1C4);
        assert_ne!(sb.incompat_flags.get() & IncompatFlags::RAID1C34.bits(), 0);
    });
}
