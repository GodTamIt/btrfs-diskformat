/// The physical address of the primary superblock.
pub const PRIMARY_SUPERBLOCK_ADDR: u64 = 0x10000;

/// The physical addresses of superblocks.
pub const SUPERBLOCK_ADDRS: [u64; 3] = [PRIMARY_SUPERBLOCK_ADDR, 0x4000000, 0x4000000000];

pub const MAGIC: u64 = 0x4D5F53665248425F;

/// Corresponds to `BTRFS_CSUM_SIZE`.
pub const CSUM_SIZE: usize = 32;

/// Corresponds to `BTRFS_FSID_SIZE`.
pub const FSID_SIZE: usize = 16;

/// Corresponds to `BTRFS_LABEL_SIZE`.
pub const LABEL_SIZE: usize = 256;

/// Corresponds to `BTRFS_UUID_SIZE`.
pub const UUID_SIZE: usize = 16;

/// Corresponds to `BTRFS_SYSTEM_CHUNK_ARRAY_SIZE`.
pub const MAX_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;

/// Corresponds to `BTRFS_NUM_BACKUP_ROOTS`.
pub const NUM_BACKUP_ROOTS: usize = 4;

/// The root tree, which locates other trees and records subvolume relationships.
pub const ROOT_TREE_OBJECTID: u64 = 1;

/// The extent allocation tree.
pub const EXTENT_TREE_OBJECTID: u64 = 2;

/// The logical-to-physical chunk mapping tree.
pub const CHUNK_TREE_OBJECTID: u64 = 3;

/// The device extent tree.
pub const DEV_TREE_OBJECTID: u64 = 4;

/// The top-level filesystem tree.
pub const FS_TREE_OBJECTID: u64 = 5;

/// The directory within the root tree, not a separate B-tree.
pub const ROOT_TREE_DIR_OBJECTID: u64 = 6;

/// The data checksum tree.
pub const CSUM_TREE_OBJECTID: u64 = 7;

/// The quota group tree.
pub const QUOTA_TREE_OBJECTID: u64 = 8;

/// The subvolume UUID lookup tree.
pub const UUID_TREE_OBJECTID: u64 = 9;

/// The free-space tracking tree.
pub const FREE_SPACE_TREE_OBJECTID: u64 = 10;

/// The block group tree.
pub const BLOCK_GROUP_TREE_OBJECTID: u64 = 11;

/// The experimental RAID stripe mapping tree.
pub const RAID_STRIPE_TREE_OBJECTID: u64 = 12;

/// The experimental remapping tree.
pub const REMAP_TREE_OBJECTID: u64 = 13;

/// The object ID used for device items in the chunk tree.
pub const DEV_ITEMS_OBJECTID: u64 = 1;

/// The object ID used for chunk items in the chunk tree.
pub const FIRST_CHUNK_TREE_OBJECTID: u64 = 256;

/// The object ID used for persistent device statistics.
pub const DEV_STATS_OBJECTID: u64 = 0;

/// The object ID used for extent checksum items.
pub const EXTENT_CSUM_OBJECTID: u64 = u64::MAX - 9;

/// The tree log used for recovery after a crash.
pub const TREE_LOG_OBJECTID: u64 = u64::MAX - 5;

/// The object ID used for log replay fixups.
pub const LOG_FIXUP_OBJECTID: u64 = u64::MAX - 6;

/// The metadata relocation tree.
pub const TREE_RELOC_OBJECTID: u64 = u64::MAX - 7;

/// The data relocation tree.
pub const DATA_RELOC_TREE_OBJECTID: u64 = u64::MAX - 8;

/// The object ID used for orphan root tracking.
pub const ORPHAN_OBJECTID: u64 = u64::MAX - 4;

/// The first root object ID available for file trees.
pub const FIRST_FREE_OBJECTID: u64 = 256;

/// The last root object ID available for file trees; see [reserved object IDs].
///
/// [reserved object IDs]: https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#reserved-objectids
pub const LAST_FREE_OBJECTID: u64 = u64::MAX - 255;
