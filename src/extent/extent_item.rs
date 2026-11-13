use crate::Key;
use bitflags::bitflags;
use static_assertions::const_assert_eq;
use zerocopy::little_endian::U64 as U64LE;
use zerocopy_derive::*;

/// The allocation-record prefix shared by extent items and skinny metadata items.
///
/// Tracks explicit references to an allocated data extent or metadata tree block.
/// Variable-length back references follow this prefix; for a non-skinny extent item
/// with [`ExtentFlags::TREE_BLOCK`] set, [`TreeBlockInfo`] comes first.
/// Skinny metadata items omit that structure and store the block level in their key.
///
/// # Resources
///
/// - [Extent items](https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#extent-item-a8)
/// - [Extent and metadata item descriptions](https://raw.githubusercontent.com/btrfs/btrfs-dev-docs/master/tree-items.txt)
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct ExtentItem {
    /// The number of explicit references to this extent.
    pub refs: U64LE,

    /// The transaction ID that allocated this extent.
    pub generation: U64LE,

    /// Extent type and back-reference properties, interpreted as [`ExtentFlags`].
    pub flags: U64LE,
}
const_assert_eq!(core::mem::size_of::<ExtentItem>(), 24);

bitflags! {
    /// Extent type and back-reference properties recorded in [`ExtentItem::flags`].
    pub struct ExtentFlags: u64 {
        /// The extent contains file data.
        const DATA = 0x1;
        /// The extent contains a metadata tree block.
        const TREE_BLOCK = 0x2;
        /// The tree block uses a full back reference.
        const FULL_BACKREF = 0x80;
    }
}

/// Additional metadata for a non-skinny tree-block extent item.
///
/// Immediately follows [`ExtentItem`] when [`ExtentFlags::TREE_BLOCK`] is set,
/// before the inline back references. Skinny metadata items omit this structure.
///
/// # Resources
///
/// - [Tree block information](https://archive.kernel.org/oldwiki/btrfs.wiki.kernel.org/index.php/Data_Structures.html#btrfs_tree_block_info)
#[derive(Copy, Clone, Debug, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct TreeBlockInfo {
    /// The first key in the tree block; currently unused.
    pub key: Key,

    /// The tree block's level in the b-tree.
    pub level: u8,
}
const_assert_eq!(core::mem::size_of::<TreeBlockInfo>(), 18);
