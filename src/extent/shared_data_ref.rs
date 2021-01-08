use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32},
};

/// This structure contains the reference count for a shared back reference for a file data extent.
///
/// This immediately follows an [ExtentInlineRef](crate::ExtentInlineRef) of type
/// [SharedDataRef](crate::ExtentInlineRefType::SharedDataRef) inside an extent item.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct SharedDataRef {
    pub count: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<SharedDataRef>(), 4);
