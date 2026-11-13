use bitflags::bitflags;
use static_assertions::const_assert_eq;

bitflags! {
    /// Features that must be supported to write to the filesystem.
    pub struct CompatRoFlags: u64 {
        /// Free space is tracked in a free space tree.
        const FREE_SPACE_TREE = 0x1;

        /// The free space tree contains valid information.
        const FREE_SPACE_TREE_VALID = 0x2;

        /// Files can carry fs-verity metadata for integrity verification.
        const VERITY = 0x4;

        /// Block groups are tracked in a separate block group tree.
        const BLOCK_GROUP_TREE = 0x8;
    }
}
const_assert_eq!(core::mem::size_of::<CompatRoFlags>(), 8);

bitflags! {
    /// Features that must be supported to use the filesystem.
    pub struct IncompatFlags: u64 {
        /// Mixed backreferences are used.
        const MIXED_BACKREF = 0x1;

        /// The filesystem has an explicitly selected default subvolume.
        const DEFAULT_SUBVOL = 0x2;

        /// Data and metadata share block groups.
        const MIXED_GROUPS = 0x4;

        /// LZO compression is used.
        const COMPRESS_LZO = 0x8;

        /// Zstandard compression is used.
        const COMPRESS_ZSTD = 0x10;

        /// Metadata blocks may be larger than the system page size.
        const BIG_METADATA = 0x20;

        /// Extended inode references are used.
        const EXTENDED_IREF = 0x40;

        /// RAID5 or RAID6 block groups are used.
        const RAID56 = 0x80;

        /// Reduced-size metadata extent references are used.
        const SKINNY_METADATA = 0x100;

        /// File holes do not require explicit extent items.
        const NO_HOLES = 0x200;

        /// Metadata block headers use the superblock's metadata UUID.
        const METADATA_UUID = 0x400;

        /// RAID1C3 or RAID1C4 block groups are used.
        const RAID1C34 = 0x800;

        /// Zoned device support is enabled.
        const ZONED = 0x1000;

        /// Experimental extent tree v2 feature bit.
        const EXTENT_TREE_V2 = 0x2000;

        /// Experimental RAID stripe tree feature bit.
        const RAID_STRIPE_TREE = 0x4000;

        /// Simple quotas are enabled.
        const SIMPLE_QUOTA = 0x10000;

        /// Experimental remap tree feature bit.
        const REMAP_TREE = 0x20000;
    }
}
const_assert_eq!(core::mem::size_of::<IncompatFlags>(), 8);
