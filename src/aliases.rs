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

// dev
pub type btrfs_dev_extent = DevExtent;

// extent
pub type btrfs_block_group_item = BlockGroupItem;
pub type btrfs_extent_data_ref = ExtentDataRef;
pub type btrfs_extent_inline_ref = ExtentInlineRef;
pub type btrfs_shared_data_ref = SharedDataRef;

pub const BTRFS_BLOCK_GROUP_RAID0: u64 = ReplicationPolicy::RAID0.bits();
