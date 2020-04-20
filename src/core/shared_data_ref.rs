use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32},
};

// TODO: fix documentation!
/// This structure contains the count for a shared back reference for a file data extent.
///
/// Follows a btrfs_extent_inline_ref of type BTRFS_SHARED_DATA_REF_KEY within an EXTENT_ITEM item. It is never used separately outside of the item body.
/// It immediately follows the btrfs_extent_inline_ref structure that contains the byte offset of the metadata leaf block that contains the EXTENT_DATA item that references this extent.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct SharedDataRef {
    pub count: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<SharedDataRef>(), 4);
