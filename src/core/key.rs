use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

/// A key used to describe and locate any item in any tree.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Key {
    pub objectid: U64<LE>,
    pub r#type: u8,
    pub offset: U64<LE>,
}
const_assert_eq!(core::mem::size_of::<Key>(), 17);
