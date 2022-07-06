use {
    crate::types::UuidBytes,
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

/// Maps physical extents on an individual backing device to a chunk. This extent
/// may be the only one for a particular chunk or one of several.
///
/// It is associated with the `DEV_ITEM` item. This structure is never used
/// outside of this item.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct DevExtent {
    /// The object ID of the chunk tree that owns this extent.
    pub chunk_tree: U64<LE>,

    /// The object ID of the chunk item that references this extent.
    pub chunk_objectid: U64<LE>,

    /// The offset of the chunk item that references this extent.
    pub chunk_offset: U64<LE>,

    /// The length of this extent, in bytes.
    pub length: U64<LE>,

    /// The UUID of the chunk tree that owns this extent.
    pub chunk_tree_uuid: UuidBytes,
}
const_assert_eq!(core::mem::size_of::<DevExtent>(), 48);
