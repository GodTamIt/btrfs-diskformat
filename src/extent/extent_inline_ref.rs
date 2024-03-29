use {
    byteorder::LE,
    num_enum::{IntoPrimitive, TryFromPrimitive},
    static_assertions::const_assert_eq,
    strum::EnumIter,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

/// This acts as a header for different types of inline extent back references inside extent or
/// metadata items.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct ExtentInlineRef {
    /// The type of reference, which corresponds with a value from [`ExtentInlineRefType`].
    /// This field also determines the semantic importance of [`offset`].
    ///
    /// [`ExtentInlineRefType`]: crate::ExtentInlineRefType
    /// [`offset`]: ExtentInlineRef::offset
    pub r#type: u8,

    /// This field has different functions depending on the value of [`type`].
    ///
    /// [`type`]: ExtentInlineRef::type
    pub offset: U64<LE>,
}
const_assert_eq!(core::mem::size_of::<ExtentInlineRef>(), 9);

/// The type of [`ExtentInlineRef`].
///
/// [`ExtentInlineRef`]: crate::ExtentInlineRef
#[derive(Copy, Clone, Debug, Hash, PartialEq, EnumIter, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ExtentInlineRefType {
    /// The reference is indirect for a tree block.
    ///
    /// [`offset`] contains the object ID of the tree root that allocated the block.
    ///
    /// [`offset`]: ExtentInlineRef::offset
    TreeBlockRef = 176,

    /// The reference is shared for a tree block.
    ///
    /// [`offset`] contains the byte offset of the node one level above in the tree where this block
    /// is located.
    ///
    /// [`offset`]: ExtentInlineRef::offset
    SharedBlockRef = 182,

    /// The reference is indirect for a data extent.
    ///
    /// An `ExtentDataRef` is located immediately after the [`type`] field and overlaps the unused
    /// [`offset`] field.
    ///
    /// [`type`]: ExtentInlineRef::type
    /// [`offset`]: ExtentInlineRef::offset
    ExtentDataRef = 178,

    /// The reference is shared for a data extent.
    ///
    /// [`offset`] contains the byte offset of the metadata that contains the extent data item that
    /// describes this extent.
    ///
    /// Immediately following [`offset`] (and the end of [`ExtentInlineRef`] structure) is a
    /// [`SharedDataRef`] that contains the reference count.
    ///
    /// [`offset`]: ExtentInlineRef::offset
    /// [`ExtentInlineRef`]: crate::ExtentInlineRef
    /// [`SharedDataRef`]: crate::SharedDataRef
    SharedDataRef = 184,
}
