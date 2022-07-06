use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32},
};

/// This structure contains the reference count for a shared back reference for a file data extent.
///
/// This immediately follows an [`ExtentInlineRef`] of type [`SharedDataRef`] inside an extent item.
///
/// [`ExtentInlineRef`]: crate::ExtentInlineRef
/// [`SharedDataRef`]: crate::ExtentInlineRefType::SharedDataRef
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct SharedDataRef {
    /// The reference count.
    pub count: U32<LE>,
}
const_assert_eq!(core::mem::size_of::<SharedDataRef>(), 4);
