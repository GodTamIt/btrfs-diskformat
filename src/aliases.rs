//! This module contains aliases from the crate's types to the wiki-style snake case names for
//! convenience.

#![allow(non_camel_case_types)]

use crate::*;

// chunk
pub type btrfs_chunk = Chunk;
pub type btrfs_stripe = Stripe;

// core
pub type btrfs_dev_item = DevItem;
pub type btrfs_dir_item = DirItem;
pub type btrfs_inode_item = InodeItem;
pub type btrfs_inode_ref = InodeRef;
pub type btrfs_inode_extref = InodeExtRef;
pub type btrfs_disk_key = Key;
pub type btrfs_header = Header;
pub type btrfs_root_backup = RootBackup;
pub type btrfs_root_item = RootItem;
pub type btrfs_root_ref = RootRef;
pub type btrfs_super_block = SuperBlock;
pub type btrfs_timespec = Time;

// object IDs
pub const BTRFS_ROOT_TREE_OBJECTID: u64 = constants::ROOT_TREE_OBJECTID;
pub const BTRFS_EXTENT_TREE_OBJECTID: u64 = constants::EXTENT_TREE_OBJECTID;
pub const BTRFS_CHUNK_TREE_OBJECTID: u64 = constants::CHUNK_TREE_OBJECTID;
pub const BTRFS_DEV_TREE_OBJECTID: u64 = constants::DEV_TREE_OBJECTID;
pub const BTRFS_FS_TREE_OBJECTID: u64 = constants::FS_TREE_OBJECTID;
pub const BTRFS_ROOT_TREE_DIR_OBJECTID: u64 = constants::ROOT_TREE_DIR_OBJECTID;
pub const BTRFS_CSUM_TREE_OBJECTID: u64 = constants::CSUM_TREE_OBJECTID;
pub const BTRFS_QUOTA_TREE_OBJECTID: u64 = constants::QUOTA_TREE_OBJECTID;
pub const BTRFS_UUID_TREE_OBJECTID: u64 = constants::UUID_TREE_OBJECTID;
pub const BTRFS_FREE_SPACE_TREE_OBJECTID: u64 = constants::FREE_SPACE_TREE_OBJECTID;
pub const BTRFS_BLOCK_GROUP_TREE_OBJECTID: u64 = constants::BLOCK_GROUP_TREE_OBJECTID;
pub const BTRFS_RAID_STRIPE_TREE_OBJECTID: u64 = constants::RAID_STRIPE_TREE_OBJECTID;
pub const BTRFS_REMAP_TREE_OBJECTID: u64 = constants::REMAP_TREE_OBJECTID;
pub const BTRFS_DEV_ITEMS_OBJECTID: u64 = constants::DEV_ITEMS_OBJECTID;
pub const BTRFS_FIRST_CHUNK_TREE_OBJECTID: u64 = constants::FIRST_CHUNK_TREE_OBJECTID;
pub const BTRFS_DEV_STATS_OBJECTID: u64 = constants::DEV_STATS_OBJECTID;
pub const BTRFS_EXTENT_CSUM_OBJECTID: u64 = constants::EXTENT_CSUM_OBJECTID;
pub const BTRFS_TREE_LOG_OBJECTID: u64 = constants::TREE_LOG_OBJECTID;
pub const BTRFS_LOG_FIXUP_OBJECTID: u64 = constants::LOG_FIXUP_OBJECTID;
pub const BTRFS_TREE_RELOC_OBJECTID: u64 = constants::TREE_RELOC_OBJECTID;
pub const BTRFS_DATA_RELOC_TREE_OBJECTID: u64 = constants::DATA_RELOC_TREE_OBJECTID;
pub const BTRFS_ORPHAN_OBJECTID: u64 = constants::ORPHAN_OBJECTID;
pub const BTRFS_FIRST_FREE_OBJECTID: u64 = constants::FIRST_FREE_OBJECTID;
pub const BTRFS_LAST_FREE_OBJECTID: u64 = constants::LAST_FREE_OBJECTID;

// key types
pub const BTRFS_INODE_ITEM_KEY: u8 = KeyType::InodeItem as u8;
pub const BTRFS_INODE_REF_KEY: u8 = KeyType::InodeRef as u8;
pub const BTRFS_INODE_EXTREF_KEY: u8 = KeyType::InodeExtRef as u8;
pub const BTRFS_XATTR_ITEM_KEY: u8 = KeyType::XattrItem as u8;
pub const BTRFS_VERITY_DESC_ITEM_KEY: u8 = KeyType::VerityDesc as u8;
pub const BTRFS_VERITY_MERKLE_ITEM_KEY: u8 = KeyType::VerityMerkle as u8;
pub const BTRFS_ORPHAN_ITEM_KEY: u8 = KeyType::OrphanItem as u8;
pub const BTRFS_DIR_LOG_ITEM_KEY: u8 = KeyType::DirLogItem as u8;
pub const BTRFS_DIR_LOG_INDEX_KEY: u8 = KeyType::DirLogIndex as u8;
pub const BTRFS_DIR_ITEM_KEY: u8 = KeyType::DirItem as u8;
pub const BTRFS_DIR_INDEX_KEY: u8 = KeyType::DirIndex as u8;
pub const BTRFS_EXTENT_DATA_KEY: u8 = KeyType::ExtentData as u8;
pub const BTRFS_CSUM_ITEM_KEY: u8 = KeyType::CsumItem as u8;
pub const BTRFS_EXTENT_CSUM_KEY: u8 = KeyType::ExtentCsum as u8;
pub const BTRFS_ROOT_ITEM_KEY: u8 = KeyType::RootItem as u8;
pub const BTRFS_ROOT_BACKREF_KEY: u8 = KeyType::RootBackref as u8;
pub const BTRFS_ROOT_REF_KEY: u8 = KeyType::RootRef as u8;
pub const BTRFS_EXTENT_ITEM_KEY: u8 = KeyType::ExtentItem as u8;
pub const BTRFS_METADATA_ITEM_KEY: u8 = KeyType::MetadataItem as u8;
pub const BTRFS_EXTENT_OWNER_REF_KEY: u8 = KeyType::ExtentOwnerRef as u8;
pub const BTRFS_EXTENT_REF_V0_KEY: u8 = KeyType::ExtentRefV0 as u8;
pub const BTRFS_BLOCK_GROUP_ITEM_KEY: u8 = KeyType::BlockGroupItem as u8;
pub const BTRFS_FREE_SPACE_INFO_KEY: u8 = KeyType::FreeSpaceInfo as u8;
pub const BTRFS_FREE_SPACE_EXTENT_KEY: u8 = KeyType::FreeSpaceExtent as u8;
pub const BTRFS_FREE_SPACE_BITMAP_KEY: u8 = KeyType::FreeSpaceBitmap as u8;
pub const BTRFS_DEV_EXTENT_KEY: u8 = KeyType::DevExtent as u8;
pub const BTRFS_DEV_ITEM_KEY: u8 = KeyType::DevItem as u8;
pub const BTRFS_CHUNK_ITEM_KEY: u8 = KeyType::ChunkItem as u8;
pub const BTRFS_RAID_STRIPE_KEY: u8 = KeyType::RaidStripe as u8;
pub const BTRFS_IDENTITY_REMAP_KEY: u8 = KeyType::IdentityRemap as u8;
pub const BTRFS_REMAP_KEY: u8 = KeyType::Remap as u8;
pub const BTRFS_REMAP_BACKREF_KEY: u8 = KeyType::RemapBackref as u8;
pub const BTRFS_QGROUP_STATUS_KEY: u8 = KeyType::QgroupStatus as u8;
pub const BTRFS_QGROUP_INFO_KEY: u8 = KeyType::QgroupInfo as u8;
pub const BTRFS_QGROUP_LIMIT_KEY: u8 = KeyType::QgroupLimit as u8;
pub const BTRFS_QGROUP_RELATION_KEY: u8 = KeyType::QgroupRelation as u8;
pub const BTRFS_TEMPORARY_ITEM_KEY: u8 = KeyType::TemporaryItem as u8;
pub const BTRFS_PERSISTENT_ITEM_KEY: u8 = KeyType::PersistentItem as u8;
pub const BTRFS_DEV_REPLACE_KEY: u8 = KeyType::DevReplace as u8;
pub const BTRFS_UUID_KEY_SUBVOL: u8 = KeyType::UuidSubvol as u8;
pub const BTRFS_UUID_KEY_RECEIVED_SUBVOL: u8 = KeyType::UuidReceivedSubvol as u8;
pub const BTRFS_STRING_ITEM_KEY: u8 = KeyType::StringItem as u8;

// superblock features
pub const BTRFS_FEATURE_COMPAT_RO_FREE_SPACE_TREE: u64 = CompatRoFlags::FREE_SPACE_TREE.bits();
pub const BTRFS_FEATURE_COMPAT_RO_FREE_SPACE_TREE_VALID: u64 =
    CompatRoFlags::FREE_SPACE_TREE_VALID.bits();
pub const BTRFS_FEATURE_COMPAT_RO_VERITY: u64 = CompatRoFlags::VERITY.bits();
pub const BTRFS_FEATURE_COMPAT_RO_BLOCK_GROUP_TREE: u64 = CompatRoFlags::BLOCK_GROUP_TREE.bits();

pub const BTRFS_FEATURE_INCOMPAT_MIXED_BACKREF: u64 = IncompatFlags::MIXED_BACKREF.bits();
pub const BTRFS_FEATURE_INCOMPAT_DEFAULT_SUBVOL: u64 = IncompatFlags::DEFAULT_SUBVOL.bits();
pub const BTRFS_FEATURE_INCOMPAT_MIXED_GROUPS: u64 = IncompatFlags::MIXED_GROUPS.bits();
pub const BTRFS_FEATURE_INCOMPAT_COMPRESS_LZO: u64 = IncompatFlags::COMPRESS_LZO.bits();
pub const BTRFS_FEATURE_INCOMPAT_COMPRESS_ZSTD: u64 = IncompatFlags::COMPRESS_ZSTD.bits();
pub const BTRFS_FEATURE_INCOMPAT_BIG_METADATA: u64 = IncompatFlags::BIG_METADATA.bits();
pub const BTRFS_FEATURE_INCOMPAT_EXTENDED_IREF: u64 = IncompatFlags::EXTENDED_IREF.bits();
pub const BTRFS_FEATURE_INCOMPAT_RAID56: u64 = IncompatFlags::RAID56.bits();
pub const BTRFS_FEATURE_INCOMPAT_SKINNY_METADATA: u64 = IncompatFlags::SKINNY_METADATA.bits();
pub const BTRFS_FEATURE_INCOMPAT_NO_HOLES: u64 = IncompatFlags::NO_HOLES.bits();
pub const BTRFS_FEATURE_INCOMPAT_METADATA_UUID: u64 = IncompatFlags::METADATA_UUID.bits();
pub const BTRFS_FEATURE_INCOMPAT_RAID1C34: u64 = IncompatFlags::RAID1C34.bits();
pub const BTRFS_FEATURE_INCOMPAT_ZONED: u64 = IncompatFlags::ZONED.bits();
pub const BTRFS_FEATURE_INCOMPAT_EXTENT_TREE_V2: u64 = IncompatFlags::EXTENT_TREE_V2.bits();
pub const BTRFS_FEATURE_INCOMPAT_RAID_STRIPE_TREE: u64 = IncompatFlags::RAID_STRIPE_TREE.bits();
pub const BTRFS_FEATURE_INCOMPAT_SIMPLE_QUOTA: u64 = IncompatFlags::SIMPLE_QUOTA.bits();
pub const BTRFS_FEATURE_INCOMPAT_REMAP_TREE: u64 = IncompatFlags::REMAP_TREE.bits();

pub const BTRFS_CSUM_TYPE_CRC32: u16 = ChecksumType::CRC32C as u16;
pub const BTRFS_CSUM_TYPE_XXHASH: u16 = ChecksumType::XXHASH64 as u16;
pub const BTRFS_CSUM_TYPE_SHA256: u16 = ChecksumType::SHA256 as u16;
pub const BTRFS_CSUM_TYPE_BLAKE2: u16 = ChecksumType::BLAKE2b as u16;

pub const BTRFS_INODE_NODATASUM: u64 = InodeFlags::NO_DATA_SUM.bits();
pub const BTRFS_INODE_NODATACOW: u64 = InodeFlags::NO_DATA_COW.bits();
pub const BTRFS_INODE_READONLY: u64 = InodeFlags::READ_ONLY.bits();
pub const BTRFS_INODE_NOCOMPRESS: u64 = InodeFlags::NO_COMPRESS.bits();
pub const BTRFS_INODE_PREALLOC: u64 = InodeFlags::PREALLOC.bits();
pub const BTRFS_INODE_SYNC: u64 = InodeFlags::SYNC.bits();
pub const BTRFS_INODE_IMMUTABLE: u64 = InodeFlags::IMMUTABLE.bits();
pub const BTRFS_INODE_APPEND: u64 = InodeFlags::APPEND.bits();
pub const BTRFS_INODE_NODUMP: u64 = InodeFlags::NO_DUMP.bits();
pub const BTRFS_INODE_NOATIME: u64 = InodeFlags::NO_ATIME.bits();
pub const BTRFS_INODE_DIRSYNC: u64 = InodeFlags::DIR_SYNC.bits();
pub const BTRFS_INODE_COMPRESS: u64 = InodeFlags::COMPRESS.bits();

// dev
pub type btrfs_dev_extent = DevExtent;

// extent
pub type btrfs_block_group_item = BlockGroupItem;
pub type btrfs_extent_data_ref = ExtentDataRef;
pub type btrfs_extent_inline_ref = ExtentInlineRefHeader;
pub type btrfs_extent_item = ExtentItem;
pub type btrfs_extent_owner_ref = ExtentOwnerRef;
pub type btrfs_file_extent_item = FileExtentItem;
pub type btrfs_free_space_info = FreeSpaceInfo;
pub type btrfs_shared_data_ref = SharedDataRef;
pub type btrfs_tree_block_info = TreeBlockInfo;

pub const BTRFS_EXTENT_FLAG_DATA: u64 = ExtentFlags::DATA.bits();
pub const BTRFS_EXTENT_FLAG_TREE_BLOCK: u64 = ExtentFlags::TREE_BLOCK.bits();
pub const BTRFS_BLOCK_FLAG_FULL_BACKREF: u64 = ExtentFlags::FULL_BACKREF.bits();

pub const BTRFS_COMPRESS_NONE: u8 = CompressionType::None as u8;
pub const BTRFS_COMPRESS_ZLIB: u8 = CompressionType::Zlib as u8;
pub const BTRFS_COMPRESS_LZO: u8 = CompressionType::Lzo as u8;
pub const BTRFS_COMPRESS_ZSTD: u8 = CompressionType::Zstd as u8;

pub const BTRFS_FILE_EXTENT_INLINE: u8 = FileExtentType::Inline as u8;
pub const BTRFS_FILE_EXTENT_REG: u8 = FileExtentType::Regular as u8;
pub const BTRFS_FILE_EXTENT_PREALLOC: u8 = FileExtentType::Prealloc as u8;

pub const BTRFS_BLOCK_GROUP_DATA: u64 = AllocationType::DATA.bits();
pub const BTRFS_BLOCK_GROUP_SYSTEM: u64 = AllocationType::SYSTEM.bits();
pub const BTRFS_BLOCK_GROUP_METADATA: u64 = AllocationType::METADATA.bits();
pub const BTRFS_BLOCK_GROUP_RAID0: u64 = ReplicationPolicy::RAID0.bits();
pub const BTRFS_BLOCK_GROUP_RAID1: u64 = ReplicationPolicy::RAID1.bits();
pub const BTRFS_BLOCK_GROUP_DUP: u64 = ReplicationPolicy::DUP.bits();
pub const BTRFS_BLOCK_GROUP_RAID10: u64 = ReplicationPolicy::RAID10.bits();
pub const BTRFS_BLOCK_GROUP_RAID5: u64 = ReplicationPolicy::RAID5.bits();
pub const BTRFS_BLOCK_GROUP_RAID6: u64 = ReplicationPolicy::RAID6.bits();
pub const BTRFS_BLOCK_GROUP_RAID1C3: u64 = ReplicationPolicy::RAID1C3.bits();
pub const BTRFS_BLOCK_GROUP_RAID1C4: u64 = ReplicationPolicy::RAID1C4.bits();

pub const BTRFS_TREE_BLOCK_REF_KEY: u8 = ExtentInlineRefType::TreeBlockRef as u8;
pub const BTRFS_SHARED_BLOCK_REF_KEY: u8 = ExtentInlineRefType::SharedBlockRef as u8;
pub const BTRFS_EXTENT_DATA_REF_KEY: u8 = ExtentInlineRefType::ExtentDataRef as u8;
pub const BTRFS_SHARED_DATA_REF_KEY: u8 = ExtentInlineRefType::SharedDataRef as u8;

// quota
pub type btrfs_qgroup_status_item = QgroupStatusItem;
pub type btrfs_qgroup_info_item = QgroupInfoItem;
pub type btrfs_qgroup_limit_item = QgroupLimitItem;

pub const BTRFS_QGROUP_STATUS_FLAG_ON: u64 = QgroupStatusFlags::ON.bits();
pub const BTRFS_QGROUP_STATUS_FLAG_RESCAN: u64 = QgroupStatusFlags::RESCAN.bits();
pub const BTRFS_QGROUP_STATUS_FLAG_INCONSISTENT: u64 = QgroupStatusFlags::INCONSISTENT.bits();
pub const BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE: u64 = QgroupStatusFlags::SIMPLE_MODE.bits();

pub const BTRFS_QGROUP_LIMIT_MAX_RFER: u64 = QgroupLimitFlags::MAX_RFER.bits();
pub const BTRFS_QGROUP_LIMIT_MAX_EXCL: u64 = QgroupLimitFlags::MAX_EXCL.bits();
pub const BTRFS_QGROUP_LIMIT_RSV_RFER: u64 = QgroupLimitFlags::RSV_RFER.bits();
pub const BTRFS_QGROUP_LIMIT_RSV_EXCL: u64 = QgroupLimitFlags::RSV_EXCL.bits();
pub const BTRFS_QGROUP_LIMIT_RFER_CMPR: u64 = QgroupLimitFlags::RFER_CMPR.bits();
pub const BTRFS_QGROUP_LIMIT_EXCL_CMPR: u64 = QgroupLimitFlags::EXCL_CMPR.bits();
