use {
    crate::{constants::CSUM_SIZE, UuidBytes},
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

/// The data stored at the start of every node.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct NodeHeader {
    /// The checksum of everything after this field, including the
    /// internal/leaf node specific part.
    pub csum: [u8; CSUM_SIZE],

    /// The filesystem UUID.
    pub fs_uuid: UuidBytes,

    /// The logical address of this node.
    pub logical_address: U64<LE>,

    /// The first 7 bits represent flags.
    pub flags: [u8; 7],

    /// The backref revision.
    ///
    /// 0 (OLD) indicates an old filesystem.
    /// 1 (MIXED) indicates a new filesystem.
    pub backref_rev: u8,

    /// The chunk tree UUID.
    pub chunk_tree_uuid: UuidBytes,

    /// The generation of this node.
    pub generation: U64<LE>,

    /// The ID of the tree containing this node.
    pub tree_id: U64<LE>,

    /// The number of items held in this node.
    pub num_items: U32<LE>,

    /// The level of this node. 0 indicates it is a leaf node.
    pub level: u8,
}
const_assert_eq!(std::mem::size_of::<NodeHeader>(), 101);
