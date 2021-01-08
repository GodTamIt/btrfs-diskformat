use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U16, U64},
};

/// References a subvolume filesystem tree root. This is used for both forward and
/// backward root references.
///
/// The name of the tree is stored after the end of the struct.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct RootRef {
    /// The subtree ID.
    pub dirid: U64<LE>,

    /// The directory sequence number of the subtree entry.
    pub sequence: U64<LE>,

    /// The length of the subtree name, stored after this field.
    pub name_len: U16<LE>,
}
const_assert_eq!(std::mem::size_of::<RootRef>(), 18);
