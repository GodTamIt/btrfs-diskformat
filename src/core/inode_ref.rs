use static_assertions::const_assert_eq;
use zerocopy::little_endian::{U16 as U16LE, U64 as U64LE};
use zerocopy_derive::*;

/// References a name in a directory from an inode.
///
/// The name is stored immediately after this fixed prefix.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#inode-ref-0c>
#[derive(Copy, Clone, Debug, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct InodeRef {
    /// The index in the directory.
    pub index: U64LE,

    /// The length of the name stored after this field.
    pub name_len: U16LE,
}
const_assert_eq!(core::mem::size_of::<InodeRef>(), 10);

/// An extended inode reference, requiring the EXTENDED_IREF feature.
///
/// The name is stored immediately after this fixed prefix.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#inode-extref-0d>
#[derive(Copy, Clone, Debug, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct InodeExtRef {
    /// The object ID of the parent directory.
    pub parent_objectid: U64LE,

    /// The index in the directory.
    pub index: U64LE,

    /// The length of the name stored after this field.
    pub name_len: U16LE,
}
const_assert_eq!(core::mem::size_of::<InodeExtRef>(), 18);
