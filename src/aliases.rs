//! This module contains aliases from the crate's types to the wiki-style snake case names for
//! convenience.

#![allow(non_camel_case_types)]

use crate::*;

// chunk
pub type btrfs_chunk = Chunk;
pub type btrfs_stripe = Stripe;

// core
pub type btrfs_dev_item = DevItem;
pub type btrfs_inode_item = InodeItem;
pub type btrfs_disk_key = Key;
pub type btrfs_header = Header;
pub type btrfs_root_backup = RootBackup;
pub type btrfs_root_item = RootItem;
pub type btrfs_root_ref = RootRef;
pub type btrfs_super_block = SuperBlock;
pub type btrfs_timespec = Time;

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

pub const BTRFS_ROOT_TREE_OBJECTID: u64 = ReservedObjectId::RootTree as u64;
pub const BTRFS_EXTENT_TREE_OBJECTID: u64 = ReservedObjectId::ExtentTree as u64;
pub const BTRFS_CHUNK_TREE_OBJECTID: u64 = ReservedObjectId::ChunkTree as u64;
pub const BTRFS_DEV_TREE_OBJECTID: u64 = ReservedObjectId::DevTree as u64;
pub const BTRFS_ROOT_TREE_DIR_OBJECTID: u64 = ReservedObjectId::RootTreeDirectory as u64;
pub const BTRFS_FS_TREE_OBJECTID: u64 = ReservedObjectId::FsTree as u64;
pub const BTRFS_CSUM_TREE_OBJECTID: u64 = ReservedObjectId::ChecksumTree as u64;
pub const BTRFS_BALANCE_OBJECTID: u64 = ReservedObjectId::Balance as u64;
pub const BTRFS_QUOTA_TREE_OBJECTID: u64 = ReservedObjectId::QuotaTree as u64;
pub const BTRFS_UUID_TREE_OBJECTID: u64 = ReservedObjectId::UuidTree as u64;
pub const BTRFS_FREE_SPACE_TREE_OBJECTID: u64 = ReservedObjectId::FreeSpaceTree as u64;
pub const BTRFS_ORPHAN_OBJECTID: u64 = ReservedObjectId::Orphan as u64;
pub const BTRFS_TREE_LOG_OBJECTID: u64 = ReservedObjectId::TreeLogTree as u64;
pub const BTRFS_TREE_RELOC_OBJECTID: u64 = ReservedObjectId::TreeRelocationTree as u64;
pub const BTRFS_DATA_RELOC_TREE_OBJECTID: u64 = ReservedObjectId::DataRelocationTree as u64;
pub const BTRFS_FREE_SPACE_OBJECTID: u64 = ReservedObjectId::FreeSpace as u64;
pub const BTRFS_FREE_INO_OBJECTID: u64 = ReservedObjectId::FreeInode as u64;
pub const BTRFS_MULTIPLE_OBJECTIDS: u64 = ReservedObjectId::MultipleObjectIds as u64;

// dev
pub type btrfs_dev_extent = DevExtent;

// extent
pub type btrfs_block_group_item = BlockGroupItem;
pub type btrfs_extent_data_ref = ExtentDataRef;
pub type btrfs_extent_inline_ref = ExtentInlineRef;
pub type btrfs_shared_data_ref = SharedDataRef;

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
