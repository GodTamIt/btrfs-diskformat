//! This module contains aliases from the crate's types to the wiki-style snake case names for
//! convenience.

#![allow(non_camel_case_types)]

use crate::*;

// chunk
pub type btrfs_chunk = Chunk;
pub type btrfs_stripe = Stripe;

// core
pub type btrfs_dev_item = DevItem;
pub type btrfs_disk_key = DiskKey;
pub type btrfs_root_backup = RootBackup;
pub type btrfs_root_ref = RootRef;
pub type btrfs_shared_data_ref = SharedDataRef;
pub type btrfs_super_block = SuperBlock;

// dev
pub type btrfs_dev_extent = DevExtent;

// extent
pub type btrfs_block_group_item = BlockGroupItem;
