use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

/// Contains an indirect back reference for a file data extent.
///
/// Immediately follows an [ExtentInlineRef](crate::ExtentInlineRef). See that documentation for
/// more details.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct ExtentDataRef {
    /// The object ID for the file tree that references this extent.
    pub root: U64<LE>,

    /// The object ID of the inode that contains the extent data that references this extent.
    pub objectid: U64<LE>,

    /// The offset within the file that corresponds to this extent.
    pub offset: U64<LE>,

    /// The reference count being held.
    pub count: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<ExtentDataRef>(), 28);
