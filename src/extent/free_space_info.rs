use static_assertions::const_assert_eq;
use zerocopy::little_endian::U32 as U32LE;
use zerocopy_derive::*;

/// Free-space information for a block group (key type 198, FREE_SPACE_INFO).
///
/// FREE_SPACE_EXTENT (199) has an empty payload; its address and length are in
/// the key. FREE_SPACE_BITMAP (200) contains raw bitmap bytes, not a fixed header;
/// each bit represents one filesystem sector.
///
/// # Resources
///
/// * <https://raw.githubusercontent.com/btrfs/btrfs-dev-docs/master/tree-items.txt>
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct FreeSpaceInfo {
    /// The number of free-space extents.
    pub extent_count: U32LE,

    /// Raw free-space information flags.
    pub flags: U32LE,
}
const_assert_eq!(core::mem::size_of::<FreeSpaceInfo>(), 8);
