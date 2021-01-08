use {
    crate::{constants::CSUM_SIZE, Key, UuidBytes},
    byteorder::LE,
    num_enum::{IntoPrimitive, TryFromPrimitive},
    static_assertions::const_assert_eq,
    strum::EnumIter,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

/// The data stored at the start of every node.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Header {
    /// The checksum of everything after this field, including the
    /// internal/leaf node specific part.
    pub csum: [u8; CSUM_SIZE],

    /// The filesystem UUID.
    pub fs_uuid: UuidBytes,

    /// The logical address of this node.
    pub logical_address: U64<LE>,

    /// The first 7 bits represent flags.
    pub flags: [u8; 7],

    /// The backref revision, which maps to a [BackrefRevision] value.
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
const_assert_eq!(std::mem::size_of::<Header>(), 101);

/// For internal (non-leaf) nodes, the node header is followed by a dynamic amount of key pointers.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct KeyPointer {
    pub key: Key,
    pub block_pointer: U64<LE>,
    pub generation: U64<LE>,
}
const_assert_eq!(std::mem::size_of::<KeyPointer>(), 33);

/// For leaf nodes, the node header is followed by a dynamic number of items.
///
/// The item data is stored at the end of the node, as pointed to by the [offset](Item::offset) and
/// [size](Item::size). The contents of the item are specified in the [key](Item::key).
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Item {
    /// The key that contains the ID and contents of this [Item].
    pub key: Key,

    /// Offset relative to the end of the header.
    pub offset: U32<LE>,

    /// The size of the data.
    pub size: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<Item>(), 25);

#[derive(Copy, Clone, Debug, Hash, PartialEq, EnumIter, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BackrefRevision {
    /// Indicates asn old filesystem.
    Old = 0,

    /// Indicates a new filesystem.
    Mixed = 1,
}
