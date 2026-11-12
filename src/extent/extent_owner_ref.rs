use static_assertions::const_assert_eq;
use zerocopy::little_endian::U64 as U64LE;
use zerocopy_derive::*;

/// The subvolume that first allocated an extent, used for simple quota accounting.
///
/// For an inline owner reference, this root occupies the offset field of
/// [`ExtentInlineRefHeader`](crate::ExtentInlineRefHeader), following the reference type byte.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/Qgroups.html#simple-quotas-squota>
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct ExtentOwnerRef {
    /// The object ID of the subvolume that owns the extent.
    pub root: U64LE,
}
const_assert_eq!(core::mem::size_of::<ExtentOwnerRef>(), 8);
