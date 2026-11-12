use crate::Key;
use static_assertions::const_assert_eq;
use zerocopy::little_endian::{U16 as U16LE, U64 as U64LE};
use zerocopy_derive::*;

/// The fixed prefix of a directory entry or extended attribute.
///
/// DIR_ITEM and DIR_INDEX use the same layout, as does XATTR_ITEM.
/// The name follows this prefix, followed by any attribute data. Each entry
/// occupies `30 + name_len + data_len` bytes. DIR_ITEM and XATTR_ITEM can contain
/// multiple entries; DIR_INDEX contains only one.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#dir-item-54>
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#xattr-item-18>
#[derive(Copy, Clone, Debug, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct DirItem {
    /// The key of the referenced item; unused and zeroed for extended attributes.
    pub location: Key,

    /// The transaction ID.
    pub transid: U64LE,

    /// The length of the attribute data; zero for directory entries.
    pub data_len: U16LE,

    /// The length of the name stored after this prefix.
    pub name_len: U16LE,

    /// The type of the referenced item.
    pub item_type: u8,
}
const_assert_eq!(core::mem::size_of::<DirItem>(), 30);
