use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

/// Defines the location, properties, and usage of a block group.
///
/// It is associated with the `BLOCK_GROUP_ITEM` and is never used outside of this
/// item.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct BlockGroupItem {
    /// The space used, in bytes, in this block group.
    pub used: U64<LE>,

    /// The object ID of the chunk backing this block group.
    pub chunk_objectid: U64<LE>,

    /// Flags indicating allocation type and replication policy.
    pub flags: U64<LE>,
}
const_assert_eq!(std::mem::size_of::<BlockGroupItem>(), 24);
